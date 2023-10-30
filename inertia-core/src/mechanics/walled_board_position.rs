use serde::Deserialize;
use serde::Serialize;
use typeshare::typeshare;

use crate::mechanics::ActorSquares;
use crate::mechanics::Square;
use crate::mechanics::WalledBoard;
use crate::solvers::SolutionStep;

use super::MoveBoard;

#[typeshare]
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct WalledBoardPosition {
  pub walled_board: WalledBoard,
  pub actor_squares: ActorSquares,
  pub goal: Square,
}

impl WalledBoardPosition {
  pub fn is_solution(&self, solution: &[SolutionStep]) -> bool {
    let actor_squares = self.apply_solution(solution);

    actor_squares.0.contains(&self.goal)
  }

  pub fn apply_solution(&self, solution: &[SolutionStep]) -> ActorSquares {
    let move_board = MoveBoard::from(&self.walled_board);
    let mut actor_squares = self.actor_squares;
    for &SolutionStep {
      actor: actor_index,
      direction,
    } in solution
    {
      let actor_square = actor_squares.0[actor_index];
      let move_destination =
        move_board.get_move_destination(actor_square, actor_squares, direction);
      actor_squares.0[actor_index] = move_destination;
    }
    return actor_squares;
  }
}

pub trait DynCloneWalledBoardPositionGenerator {
  fn clone_dyn(&self) -> Box<dyn WalledBoardPositionGenerator>;
}

impl<T> DynCloneWalledBoardPositionGenerator for T
where
  T: 'static + WalledBoardPositionGenerator + Clone,
{
  fn clone_dyn(&self) -> Box<dyn WalledBoardPositionGenerator> {
    Box::new(self.clone())
  }
}

pub trait WalledBoardPositionGenerator:
  DynCloneWalledBoardPositionGenerator + std::fmt::Debug + Send + Sync
{
  fn generate_position(&self) -> WalledBoardPosition;
}

impl Clone for Box<dyn WalledBoardPositionGenerator> {
  fn clone(&self) -> Self {
    self.clone_dyn()
  }
}
