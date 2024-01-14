use std::net::SocketAddr;

use axum::extract::ws::Message;
use axum::extract::ws::WebSocket;
use futures::stream::SplitStream;
use futures::StreamExt;
use inertia_core::message::FromClientMessage;
use inertia_core::message::JoinMessage;
use inertia_core::message::ToClientMessage;
use inertia_core::solvers::Difficulty;
use inertia_core::state::data::PlayerId;
use inertia_core::state::data::PlayerName;
use inertia_core::state::data::RoomId;
use inertia_core::state::event::apply_event::RoomEvent;
use inertia_core::state::event::connect::Connect;
use tokio::sync::broadcast;

use crate::difficulty_board_generator::DifficultyDbBoardGenerator;
use crate::state::AppState;

#[derive(Debug)]
pub enum JoinError {
  NoMessageReceived,
  ConnectionClosed,
  WebSocketError(axum::Error),
}

pub struct JoinInfo {
  pub room_id: RoomId,
  pub player_id: PlayerId,
  pub player_name: PlayerName,
  pub broadcast_channel_sender: broadcast::Sender<ToClientMessage>,
  pub broadcast_channel_receiver: broadcast::Receiver<ToClientMessage>,
}

pub async fn join(
  ws_receiver: &mut SplitStream<WebSocket>,
  socket_address: &SocketAddr,
  state: &AppState,
) -> Result<JoinInfo, JoinError> {
  macro_rules! reject {
    ($($t:tt)*) => (tracing::debug!("WebSocket [{}]: Join rejected. {}", socket_address, format_args!($($t)*)))
  }

  loop {
    let msg = ws_receiver
      .next()
      .await
      .ok_or(JoinError::NoMessageReceived)?
      .map_err(JoinError::WebSocketError)?;

    let msg = match msg {
      Message::Text(msg) => msg,
      Message::Close(_) => return Err(JoinError::ConnectionClosed),
      _ => {
        reject!("Unexpected message: {:?}", msg);
        continue;
      }
    };

    let msg = match serde_json::from_str::<FromClientMessage>(&msg) {
      Ok(msg) => msg,
      Err(err) => {
        reject!(
          "Failed to parse message. Message: {:?}, Error: {}",
          msg,
          err
        );
        continue;
      }
    };

    let join_message = match msg {
      FromClientMessage::Join(join_message) => join_message,
      _ => {
        reject!("Unexpected message: {:?}", msg);
        continue;
      }
    };

    let JoinMessage {
      player_id,
      player_name,
      player_reconnect_key,
      room_id,
      min_difficulty,
      max_difficulty,
    } = join_message;

    state
      .ensure_room_exists(
        room_id,
        DifficultyDbBoardGenerator::new(
          state.db_pool.clone(),
          min_difficulty.unwrap_or(Difficulty::Easiest),
          max_difficulty.unwrap_or(Difficulty::Hard),
        ),
      )
      .await;

    tracing::debug!(
      "WebSocket [{}]: Joining room {:?} with difficulty {:?} -> {:?}",
      socket_address,
      room_id,
      min_difficulty,
      max_difficulty
    );

    let (broadcast_channel_sender, broadcast_channel_receiver) =
      match state.get_broadcast_channel_pair(room_id).await {
        Ok(result) => result,
        Err(err) => {
          reject!("Error during connection: {:?}", err);
          continue;
        }
      };

    let connect_event = RoomEvent::Connect(Connect {
      player_name: player_name.clone(),
      player_id,
      player_reconnect_key,
    });
    if let Err(err) = state.apply_event(room_id, connect_event).await {
      reject!("Error during connection: {:?}", err);
      continue;
    };

    return Ok(JoinInfo {
      room_id,
      player_id,
      player_name,
      broadcast_channel_sender,
      broadcast_channel_receiver,
    });
  }
}
