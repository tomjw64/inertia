use crate::mechanics::ActorSquares;
use crate::mechanics::Position;
use crate::mechanics::PositionGenerator;
use crate::mechanics::Square;
use crate::mechanics::WalledBoard;

#[derive(Debug, Clone, Copy)]
pub struct EmptyMiddleGoalBoardGenerator;

impl EmptyMiddleGoalBoardGenerator {
  pub fn new() -> Self {
    Self
  }
}

impl Default for EmptyMiddleGoalBoardGenerator {
  fn default() -> Self {
    EmptyMiddleGoalBoardGenerator::new()
  }
}

impl PositionGenerator for EmptyMiddleGoalBoardGenerator {
  fn generate_position(&self) -> Position {
    Position {
      goal: Square::from_row_col(8, 8),
      actor_squares: ActorSquares([Square(0), Square(1), Square(2), Square(3)]),
      walled_board: WalledBoard::EMPTY,
    }
  }
}
