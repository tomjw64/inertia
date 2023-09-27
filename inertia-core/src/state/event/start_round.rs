use crate::state::data::RoomState;
use crate::state::data::RoundStart;
use crate::state::data::RoundSummary;

use super::result::EventResult;

pub fn round_summary_start_round(state: RoundSummary) -> EventResult {
  let RoundSummary { meta, .. } = state;
  EventResult::ok(RoomState::RoundStart(RoundStart {
    board: meta.generator.generate_position(),
    meta,
  }))
}
