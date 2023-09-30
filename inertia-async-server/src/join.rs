use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::ws::Message;
use axum::extract::ws::WebSocket;
use futures::stream::SplitStream;
use futures::StreamExt;
use inertia_core::board_generators::ClassicBoardGenerator;
use inertia_core::message::FromClientMessage;
use inertia_core::message::JoinMessage;
use inertia_core::state::data::PlayerId;
use inertia_core::state::data::PlayerName;
use inertia_core::state::data::RoomId;
use inertia_core::state::event::apply_event::RoomEvent;
use inertia_core::state::event::connect::Connect;
use tokio::sync::broadcast;

use crate::state::AppState;
use crate::state::Room;

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
  pub channel_sender: broadcast::Sender<String>,
  pub channel_receiver: broadcast::Receiver<String>,
}

pub async fn join(
  ws_receiver: &mut SplitStream<WebSocket>,
  socket_address: &SocketAddr,
  state: &Arc<AppState>,
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
    } = join_message;

    let should_create_room = state
      .rooms
      .read()
      .await
      .get(&join_message.room_id)
      .is_none();

    if should_create_room {
      state
        .rooms
        .write()
        .await
        .entry(join_message.room_id)
        .or_insert_with(|| {
          Room::new(join_message.room_id, ClassicBoardGenerator::new())
        });
    }

    let connect_event = RoomEvent::Connect(Connect {
      player_name: player_name.clone(),
      player_id,
      player_reconnect_key,
    });
    if let Err(err) = state.apply_event(room_id, connect_event).await {
      reject!("Error during connection: {:?}", err);
      continue;
    };

    let (channel_sender, channel_receiver) =
      match state.get_channel_pair(room_id).await {
        Ok(result) => result,
        Err(err) => {
          reject!("Error during connection: {:?}", err);
          continue;
        }
      };

    return Ok(JoinInfo {
      room_id,
      player_id,
      player_name,
      channel_sender,
      channel_receiver,
    });
  }
}
