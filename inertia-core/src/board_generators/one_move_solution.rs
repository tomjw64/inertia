use crate::mechanics::ActorSquares;
use crate::mechanics::Direction;
use crate::mechanics::Position;
use crate::mechanics::SolvedPosition;
use crate::mechanics::SolvedPositionGenerator;
use crate::mechanics::Square;
use crate::mechanics::WalledBoard;
use crate::solvers::Solution;
use crate::solvers::SolutionStep;

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

impl SolvedPositionGenerator for OneMoveSolutionBoardGenerator {
  fn generate_solved_position(&self) -> SolvedPosition {
    SolvedPosition {
      position: Position {
        goal: Square::from_row_col(15, 0),
        actor_squares: ActorSquares([
          Square(0),
          Square(1),
          Square(2),
          Square(3),
        ]),
        walled_board: WalledBoard::EMPTY,
      },
      solution: Solution(vec![SolutionStep {
        actor: 0,
        direction: Direction::Down,
      }]),
    }
  }
}

#[cfg(test)]
mod test {
  use crate::mechanics::SolvedPosition;
  use crate::mechanics::SolvedPositionGenerator;

  use super::OneMoveSolutionBoardGenerator;

  #[test]
  fn verify() {
    let SolvedPosition { position, solution } =
      OneMoveSolutionBoardGenerator::new().generate_solved_position();
    assert!(position.is_solution(&solution))
  }
}
