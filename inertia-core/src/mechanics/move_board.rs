use crate::mechanics::BitBoard;
use crate::mechanics::Square;

use super::BlockBoard;
use super::Direction;
use super::WalledBoard;

#[derive(Copy, Clone, Debug)]
pub struct MoveBoard {
  pub(crate) up_moves: [Square; 256],
  pub(crate) down_moves: [Square; 256],
  pub(crate) right_moves: [Square; 256],
  pub(crate) left_moves: [Square; 256],
}

impl From<&WalledBoard> for MoveBoard {
  fn from(value: &WalledBoard) -> Self {
    let mut move_board = MoveBoard {
      up_moves: [Square(0); 256],
      down_moves: [Square(0); 256],
      right_moves: [Square(0); 256],
      left_moves: [Square(0); 256],
    };
  }
}
