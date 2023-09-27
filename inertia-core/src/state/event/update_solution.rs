use thiserror::Error;

use crate::solvers::SolutionStep;
use crate::state::data::PlayerBid;
use crate::state::data::PlayerId;
use crate::state::data::RoomState;
use crate::state::data::RoundSolving;
use crate::state::data::RoundSummary;

use super::result::EventResult;

#[derive(Error, Debug)]
pub enum UpdateSolutionError {
  #[error("Given player is not the solver")]
  WrongPlayer,
  #[error("Given solution length exceeds the player bid")]
  SolutionExceedsBid,
}

#[derive(Debug)]
pub struct UpdateSolution {
  pub player_id: PlayerId,
  pub solution: Vec<SolutionStep>,
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
  } = state;
  let UpdateSolution {
    player_id,
    solution: updated_solution,
  } = event;

  if player_id != solver {
    return EventResult::err(
      RoomState::RoundSolving(RoundSolving {
        board,
        meta,
        player_bids,
        solver,
        solution: current_solution,
      }),
      UpdateSolutionError::WrongPlayer,
    );
  }

  let bid = player_bids.get(&player_id).unwrap_or(&PlayerBid::None);
  let effective_bid_value = bid.to_effective_value();

  if updated_solution.len() > effective_bid_value {
    return EventResult::err(
      RoomState::RoundSolving(RoundSolving {
        board,
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
      .entry(player_id)
      .and_modify(|info| info.player_score += 1);
    return EventResult::ok(RoomState::RoundSummary(RoundSummary {
      meta,
      last_round_board: Some(board),
      last_round_solution: Some(updated_solution),
    }));
  }

  EventResult::ok(RoomState::RoundSolving(RoundSolving {
    board,
    meta,
    player_bids,
    solver,
    solution: updated_solution,
  }))
}
