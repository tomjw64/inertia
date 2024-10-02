use crate::mechanics::Square;
use core::fmt;
use primitive_types::U256;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::ops::Not;

#[cfg(feature = "web")]
use {tsify::declare, wasm_bindgen::prelude::wasm_bindgen};

#[cfg_attr(feature = "web", declare)]
pub type ExpandedBitBoard = [bool; 256];

#[derive(Copy, Clone, PartialEq)]
pub struct BitBoard(pub(crate) U256);

impl fmt::Debug for BitBoard {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let rows = (0..16).map(|row_index| {
      let low_byte = self.0.byte(row_index * 2) as u16;
      let high_byte = self.0.byte(row_index * 2 + 1) as u16;
      low_byte + (high_byte << 8)
    });
    f.write_str("\n")?;
    for row in rows {
      let mut squares = format!("{:016b}", row)
        .chars()
        .map(|c| c.to_digit(10).unwrap().to_string())
        .collect::<Vec<_>>();
      squares.reverse();
      f.write_str("[")?;
      f.write_str(&squares.join(", "))?;
      f.write_str("]\n")?;
    }
    Ok(())
  }
}

impl From<Square> for BitBoard {
  fn from(square: Square) -> Self {
    let relevant_u64_index = square.0 / 64;
    let relevant_bit_index = square.0 % 64;
    let relevant_u64 = 1 << relevant_bit_index;
    let mut data = [0, 0, 0, 0];
    data[relevant_u64_index as usize] = relevant_u64;
    BitBoard::new(data)
  }
}

impl BitBoard {
  pub const ZERO: Self = Self(U256([0, 0, 0, 0]));
  pub const MAX: Self = Self(U256::MAX);

  pub const fn new(data: [u64; 4]) -> Self {
    Self(U256(data))
  }

  pub fn set_bit(&mut self, bit_index: usize) {
    let relevant_u64_index = bit_index / 64;
    let relevant_bit_index = bit_index % 64;
    let relevant_u64 = 1 << relevant_bit_index;
    self.0 .0[relevant_u64_index] |= relevant_u64;
  }

  pub fn bit(&self, bit_index: usize) -> bool {
    self.0.bit(bit_index)
  }

  pub fn trailing_zeros(&self) -> u32 {
    // Board is LE, but U256 is BE
    self.0.leading_zeros()
  }

  pub fn leading_zeros(&self) -> u32 {
    // Board is LE, but U256 is BE
    self.0.trailing_zeros()
  }

  pub fn to_expanded(&self) -> ExpandedBitBoard {
    let mut bools = [false; 256];
    for (index, b) in bools.iter_mut().enumerate() {
      *b = self.bit(index)
    }
    bools
  }
}

impl Not for BitBoard {
  type Output = BitBoard;

  fn not(self) -> Self::Output {
    BitBoard(self.0.not())
  }
}

impl BitOr<BitBoard> for BitBoard {
  type Output = BitBoard;

  fn bitor(self, rhs: BitBoard) -> Self::Output {
    BitBoard(self.0.bitor(rhs.0))
  }
}

impl BitOr<&BitBoard> for BitBoard {
  type Output = BitBoard;

  fn bitor(self, rhs: &BitBoard) -> Self::Output {
    BitBoard(self.0.bitor(rhs.0))
  }
}

impl BitAnd<BitBoard> for BitBoard {
  type Output = BitBoard;

  fn bitand(self, rhs: BitBoard) -> Self::Output {
    BitBoard(self.0.bitand(rhs.0))
  }
}

impl BitAnd<&BitBoard> for BitBoard {
  type Output = BitBoard;

  fn bitand(self, rhs: &BitBoard) -> Self::Output {
    BitBoard(self.0.bitand(rhs.0))
  }
}
