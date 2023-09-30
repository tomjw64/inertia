use crate::state::data::RoomState;
use crate::state::data::RoundStart;
use crate::state::data::RoundSummary;

use super::result::EventResult;

pub fn round_summary_start_round(state: RoundSummary) -> EventResult {
  let RoundSummary { mut meta, .. } = state;
  meta.round_number += 1;
  meta.player_info.iter_mut().for_each(|(_, player_info)| {
    player_info.player_last_seen = meta.round_number;
  });
  EventResult::ok(RoomState::RoundStart(RoundStart {
    board: meta.generator.generate_position(),
    meta,
  }))
}
