use crate::state::data::RoomState;
use crate::state::data::RoundBidding;
use crate::state::data::RoundSolving;
use crate::state::data::RoundSummary;

use super::result::EventResult;

pub fn round_bidding_finalize_bids(state: RoundBidding) -> EventResult {
  let RoundBidding {
    meta,
    board,
    player_bids,
  } = state;

  let next_solver = player_bids.get_next_solver();

  match next_solver {
    Some(next_solver_id) => {
      EventResult::ok(RoomState::RoundSolving(RoundSolving {
        meta,
        board,
        player_bids,
        solver: next_solver_id,
        solution: Vec::new(),
      }))
    }
    None => EventResult::ok(RoomState::RoundSummary(RoundSummary {
      meta,
      last_round_board: Some(board),
      last_round_solution: None,
      last_solver: None,
    })),
  }
}
