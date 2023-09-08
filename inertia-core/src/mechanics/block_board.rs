use crate::mechanics::BitBoard;
use crate::mechanics::Square;

#[derive(Copy, Clone, Debug)]
pub struct BlockBoard {
  pub(crate) goal: Square,
  pub(crate) up_blocks: BitBoard,
  pub(crate) down_blocks: BitBoard,
  pub(crate) right_blocks: BitBoard,
  pub(crate) left_blocks: BitBoard,
}

impl BlockBoard {
  pub(crate) const EMPTY: Self = Self {
    goal: Square(0),
    up_blocks: BitBoard::ZERO,
    down_blocks: BitBoard::ZERO,
    right_blocks: BitBoard::ZERO,
    left_blocks: BitBoard::ZERO,
  };

  pub(crate) fn has_any_block_on(&self, square: Square) -> bool {
    [
      self.up_blocks,
      self.down_blocks,
      self.left_blocks,
      self.right_blocks,
    ]
    .iter()
    .any(|bitboard| bitboard.bit(square.0 as usize))
  }
}
