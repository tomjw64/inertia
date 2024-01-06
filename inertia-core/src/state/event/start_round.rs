use crate::state::data::RoomState;
use crate::state::data::RoundStart;
use crate::state::data::RoundSummary;

use super::result::EventResult;

pub fn round_summary_start_round(state: RoundSummary) -> EventResult {
  let RoundSummary { mut meta, .. } = state;
  meta.round_number += 1;
  meta
    .player_info
    .iter_mut()
    .filter(|(_, player_info)| player_info.player_connected)
    .for_each(|(_, player_info)| {
      player_info.player_last_seen = meta.round_number;
    });
  let solved_position = meta.generator.generate_solved_position();
  EventResult::ok(RoomState::RoundStart(RoundStart {
    board: solved_position.position,
    optimal_solution: solved_position.solution,
    meta,
  }))
}
