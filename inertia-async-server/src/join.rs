use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::ws::Message;
use axum::extract::ws::WebSocket;
use futures::stream::SplitStream;
use futures::StreamExt;
use inertia_core::message::FromClientMessage;
use inertia_core::state::PlayerId;
use inertia_core::state::PlayerName;
use inertia_core::state::PlayerReconnectKey;
use inertia_core::state::RoomId;
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

    if join_message.player_name.0.is_empty() {
      reject!("Empty username.");
      continue;
    }

    if join_message.room_id.0 == 0 {
      reject!("Room ID 0.");
      continue;
    }

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
        .or_insert_with(|| Room::new(join_message.room_id));
    }

    let rooms = state.rooms.read().await;
    let room = match rooms.get(&join_message.room_id) {
      Some(room) => room,
      None => {
        reject!("Room with ID {} disappeared.", join_message.room_id.0);
        continue;
      }
    };

    let mut room_data = room.state.lock().await;

    let reconnect_key_for_id =
      room_data.player_reconnect_keys.get(&join_message.player_id);
    let id_for_username = room_data
      .players
      .iter()
      .find(|&(_, player_name)| player_name == &join_message.player_name)
      .map(|(id, _)| id);

    if id_for_username.is_some()
      && id_for_username != Some(&join_message.player_id)
    {
      reject!("Username taken: {}.", join_message.player_name.0);
      continue;
    }

    if reconnect_key_for_id.is_some()
      && reconnect_key_for_id != Some(&join_message.player_reconnect_key)
    {
      reject!(
        "Bad reconnect key: {}.",
        join_message.player_reconnect_key.0
      );
      continue;
    }

    let current_round = room_data.round_number;
    room_data
      .players
      .insert(join_message.player_id, join_message.player_name.clone());
    room_data
      .players_connected
      .insert(join_message.player_id, true);
    room_data
      .player_reconnect_keys
      .insert(join_message.player_id, join_message.player_reconnect_key);
    room_data
      .player_last_seen
      .insert(join_message.player_id, current_round);

    return Ok(JoinInfo {
      room_id: join_message.room_id,
      player_id: join_message.player_id,
      player_name: join_message.player_name,
      channel_sender: room.meta.channel.clone(),
      channel_receiver: room.meta.channel.subscribe(),
    });
  }
}
