use crate::state::data::PlayerBid;
use crate::state::data::PlayerId;
use crate::state::data::RoomState;
use crate::state::data::RoundSolving;
use crate::state::data::RoundSummary;

use super::result::EventResult;

#[derive(Debug)]
pub struct YieldSolve {
  pub player_id: PlayerId,
}

pub fn round_solving_yield_solve(state: RoundSolving) -> EventResult {
  let RoundSolving {
    meta,
    board,
    mut player_bids,
    solver,
    solution,
  } = state;

  player_bids.fail(solver);

  let next_bidder = player_bids.get_next_solver();

  match next_bidder {
    Some(next_bidder_id) => {
      EventResult::ok(RoomState::RoundSolving(RoundSolving {
        meta,
        board,
        player_bids,
        solver: next_bidder_id,
        solution,
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
