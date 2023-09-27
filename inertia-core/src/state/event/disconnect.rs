use crate::state::data::PlayerId;
use crate::state::data::RoomMeta;
use crate::state::data::RoomState;
use crate::state::data::RoundSummary;

use super::result::EventResult;

#[derive(Debug)]
pub struct SoftDisconnect {
  pub player_id: PlayerId,
}

fn room_meta_hard_disconnect(meta: &mut RoomMeta, event: SoftDisconnect) {
  let SoftDisconnect { player_id } = event;
  meta.player_info.remove(&player_id);
}

fn room_meta_soft_disconnect(meta: &mut RoomMeta, event: SoftDisconnect) {
  let SoftDisconnect { player_id } = event;
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

pub fn round_summary_soft_disconnect(
  mut state: RoundSummary,
  event: SoftDisconnect,
) -> EventResult {
  room_meta_soft_disconnect(&mut state.meta, event);
  if players_still_present(&state.meta) {
    EventResult::ok(RoomState::RoundSummary(state))
  } else {
    EventResult::ok(RoomState::Closed)
  }
}
