use thiserror::Error;

use crate::solvers::Solution;
use crate::state::data::RoomState;
use crate::state::data::RoundSolving;
use crate::state::data::RoundSummary;

use super::result::EventResult;

#[derive(Error, Debug)]
pub enum UpdateSolutionError {
  #[error("Given solution length exceeds the bid")]
  SolutionExceedsBid,
}

#[derive(Debug, Clone)]
pub struct UpdateSolution {
  pub solution: Solution,
}

pub fn round_solving_update_solution(
  state: RoundSolving,
  event: UpdateSolution,
) -> EventResult {
  let RoundSolving {
    mut meta,
    board,
    player_bids,
    solver,
    solution: current_solution,
    optimal_solution,
  } = state;
  let UpdateSolution {
    solution: updated_solution,
  } = event;

  let bid = player_bids.get(solver);
  let effective_bid_value = bid.to_effective_value();

  if updated_solution.0.len() > effective_bid_value as usize {
    return EventResult::err(
      RoomState::RoundSolving(RoundSolving {
        board,
        optimal_solution,
        meta,
        player_bids,
        solver,
        solution: current_solution,
      }),
      UpdateSolutionError::SolutionExceedsBid,
    );
  }

  if board.is_solution(&updated_solution) {
    meta
      .player_info
      .entry(solver)
      .and_modify(|info| info.player_score += 1);
    return EventResult::ok(RoomState::RoundSummary(RoundSummary {
      meta,
      last_round_board: Some(board),
      last_round_optimal_solution: Some(optimal_solution),
      last_round_solution: Some(updated_solution),
      last_solver: Some(solver),
    }));
  }

  EventResult::ok(RoomState::RoundSolving(RoundSolving {
    board,
    optimal_solution,
    meta,
    player_bids,
    solver,
    solution: updated_solution,
  }))
}
