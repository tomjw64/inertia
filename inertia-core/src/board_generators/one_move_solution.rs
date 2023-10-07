use crate::mechanics::ActorSquares;
use crate::mechanics::Square;
use crate::mechanics::WalledBoard;
use crate::mechanics::WalledBoardPosition;
use crate::mechanics::WalledBoardPositionGenerator;

#[derive(Debug, Clone, Copy)]
pub struct OneMoveSolutionBoardGenerator;

impl OneMoveSolutionBoardGenerator {
  pub fn new() -> Self {
    Self
  }
}

impl Default for OneMoveSolutionBoardGenerator {
  fn default() -> Self {
    OneMoveSolutionBoardGenerator::new()
  }
}

impl WalledBoardPositionGenerator for OneMoveSolutionBoardGenerator {
  fn generate_position(&self) -> WalledBoardPosition {
    WalledBoardPosition {
      goal: Square::from_row_col(15, 0),
      actor_squares: ActorSquares([Square(0), Square(1), Square(2), Square(3)]),
      walled_board: WalledBoard::EMPTY,
    }
  }
}

#[cfg(test)]
mod test {
  use crate::mechanics::Direction;
  use crate::mechanics::WalledBoardPositionGenerator;
  use crate::solvers::SolutionStep;

  use super::OneMoveSolutionBoardGenerator;

  #[test]
  fn verify() {
    OneMoveSolutionBoardGenerator::new()
      .generate_position()
      .is_solution(&[SolutionStep {
        actor: 0,
        direction: Direction::Down,
      }]);
  }
}
