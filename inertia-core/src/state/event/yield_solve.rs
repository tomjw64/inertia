use thiserror::Error;

use crate::state::data::PlayerBid;
use crate::state::data::PlayerId;
use crate::state::data::RoomState;
use crate::state::data::RoundSolving;
use crate::state::data::RoundSummary;

use super::result::EventResult;

#[derive(Error, Debug)]
pub enum YieldSolveError {
  #[error("Given player is not the solver")]
  WrongPlayer,
}

#[derive(Debug)]
pub struct YieldSolve {
  pub player_id: PlayerId,
}

pub fn round_solving_yield_solve(
  state: RoundSolving,
  event: YieldSolve,
) -> EventResult {
  let RoundSolving {
    meta,
    board,
    mut player_bids,
    solver,
    solution,
  } = state;
  let YieldSolve { player_id } = event;

  if player_id != solver {
    return EventResult::err(
      RoomState::RoundSolving(RoundSolving {
        board,
        meta,
        player_bids,
        solver,
        solution,
      }),
      YieldSolveError::WrongPlayer,
    );
  }

  let new_bid = player_bids
    .get(&player_id)
    .unwrap_or(&PlayerBid::None)
    .to_failed();
  player_bids.insert(player_id, new_bid);

  let next_bidder = player_bids
    .iter()
    .filter(|(_, bid)| bid.is_prospective())
    .min_by_key(|(_, bid)| bid.to_effective_value())
    .map(|(id, _)| id);

  match next_bidder {
    Some(&next_bidder_id) => {
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
    })),
  }
}
