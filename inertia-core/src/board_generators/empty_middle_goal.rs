use crate::mechanics::ActorSquares;
use crate::mechanics::Square;
use crate::mechanics::WalledBoard;
use crate::mechanics::WalledBoardPosition;
use crate::mechanics::WalledBoardPositionGenerator;

pub struct EmptyMiddleGoalBoardGenerator;

impl EmptyMiddleGoalBoardGenerator {
  pub fn new() -> Self {
    Self
  }
}

impl WalledBoardPositionGenerator for EmptyMiddleGoalBoardGenerator {
  fn generate_position(&self) -> WalledBoardPosition {
    let mut walled_board = WalledBoard::EMPTY;
    walled_board.goal = Square::from_row_col(8, 8);
    WalledBoardPosition {
      actor_squares: ActorSquares([Square(0), Square(1), Square(2), Square(3)]),
      walled_board,
    }
  }
}
