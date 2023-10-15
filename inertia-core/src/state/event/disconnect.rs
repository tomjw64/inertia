use crate::state::data::PlayerId;
use crate::state::data::RoomMeta;
use crate::state::data::RoomState;

use super::apply_event::RoomEvent;
use super::result::EventError;
use super::result::EventResult;

#[derive(Debug, Clone)]
pub struct Disconnect {
  pub player_id: PlayerId,
}

fn room_meta_hard_disconnect(meta: &mut RoomMeta, event: Disconnect) {
  let Disconnect { player_id } = event;
  meta.player_info.remove(&player_id);
}

fn room_meta_soft_disconnect(meta: &mut RoomMeta, event: Disconnect) {
  let Disconnect { player_id } = event;
  meta
    .player_info
    .entry(player_id)
    .and_modify(|e| e.player_connected = false);
}

fn players_still_present(meta: &RoomMeta) -> bool {
  meta
    .player_info
    .iter()
    .any(|(_, info)| info.player_connected)
}

pub fn hard_disconnect(mut state: RoomState, event: Disconnect) -> EventResult {
  if let Some(meta) = state.get_meta_mut() {
    room_meta_hard_disconnect(meta, event);
    if players_still_present(meta) {
      EventResult::ok(state)
    } else {
      EventResult::ok(RoomState::Closed)
    }
  } else {
    EventResult {
      error: Some(EventError::IncompatibleState(
        state.to_string(),
        RoomEvent::SoftDisconnect(event),
      )),
      result: state,
    }
  }
}

pub fn soft_disconnect(mut state: RoomState, event: Disconnect) -> EventResult {
  if let Some(meta) = state.get_meta_mut() {
    room_meta_soft_disconnect(meta, event);
    if players_still_present(meta) {
      EventResult::ok(state)
    } else {
      EventResult::ok(RoomState::Closed)
    }
  } else {
    EventResult {
      error: Some(EventError::IncompatibleState(
        state.to_string(),
        RoomEvent::SoftDisconnect(event),
      )),
      result: state,
    }
  }
}
