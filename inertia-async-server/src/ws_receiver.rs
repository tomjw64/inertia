use std::net::SocketAddr;
use std::sync::Arc;

use inertia_core::message::FromClientMessage;
use inertia_core::state::data::PlayerId;
use inertia_core::state::data::RoomId;
use inertia_core::state::event::apply_event::RoomEvent;

use crate::state::AppState;

async fn handle_message_from_client(
  socket_address: SocketAddr,
  msg: FromClientMessage,
  state: &Arc<AppState>,
  room_id: RoomId,
  player_id: PlayerId,
) {
  macro_rules! ws_debug {
    ($($t:tt)*) => (tracing::debug!("WebSocket [{}]: {}", socket_address, format_args!($($t)*)))
  }

  match msg {
    FromClientMessage::Join(_) => ws_debug!("Unexpected join message."),
    FromClientMessage::Rename(_) => unimplemented!(),
    FromClientMessage::StartRound => {
      state.apply_event(room_id, RoomEvent::StartRound).await.ok();
    }
  };
}
