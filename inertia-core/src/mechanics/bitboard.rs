use crate::mechanics::ActorSquares;
use crate::mechanics::Square;
use core::fmt;
use std::ops::BitAnd;
use std::ops::BitAndAssign;
use std::ops::BitOr;
use std::ops::BitOrAssign;
use std::ops::Not;

#[cfg(feature = "web")]
use {tsify::declare, wasm_bindgen::prelude::wasm_bindgen};

#[cfg_attr(feature = "web", declare)]
pub type ExpandedBitBoard = [bool; 256];

#[derive(Copy, Clone, PartialEq, Eq)]

pub struct BitBoard(pub(crate) [u64; 4]);

impl From<Square> for BitBoard {
  fn from(square: Square) -> Self {
    let mut bitboard = BitBoard::ZERO;
    bitboard.set_bit(square.0);
    bitboard
  }
}

impl From<ActorSquares> for BitBoard {
  fn from(actor_squares: ActorSquares) -> Self {
    let mut bitboard = BitBoard::ZERO;
    for Square(index) in actor_squares.0 {
      bitboard.set_bit(index);
    }
    bitboard
  }
}

impl fmt::Debug for BitBoard {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let rows = (0..16).map(|row_index| {
      let low_byte = self.0[row_index * 2] as u16;
      let high_byte = self.0[row_index * 2 + 1] as u16;
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

impl BitBoard {
  pub const ZERO: Self = Self([0, 0, 0, 0]);
  pub const MAX: Self = Self([u64::MAX, u64::MAX, u64::MAX, u64::MAX]);

  pub const fn new(data: [u64; 4]) -> Self {
    Self(data)
  }

  pub fn set_bit(&mut self, bit_index: u8) {
    let relevant_u64_index = bit_index / 64;
    let relevant_bit_index = bit_index % 64;
    let relevant_u64 = 1 << relevant_bit_index;
    self.0[relevant_u64_index as usize] |= relevant_u64;
  }

  pub fn bit(&self, bit_index: u8) -> bool {
    let relevant_u64_index = bit_index / 64;
    let relevant_bit_index = bit_index % 64;
    let relevant_u64 = 1 << relevant_bit_index;
    self.0[relevant_u64_index as usize] & relevant_u64 != 0
  }

  pub fn is_superset_of<Rhs>(&self, other: Rhs) -> bool
  where
    Rhs: AsRef<[u64; 4]>,
  {
    return *self | other == *self;
  }

  pub fn to_expanded(&self) -> ExpandedBitBoard {
    let mut bools = [false; 256];
    for (index, b) in bools.iter_mut().enumerate() {
      *b = self.bit(index as u8)
    }
    bools
  }
}

impl AsRef<[u64; 4]> for BitBoard {
  fn as_ref(&self) -> &[u64; 4] {
    &self.0
  }
}

impl Not for BitBoard {
  type Output = BitBoard;

  fn not(self) -> Self::Output {
    Self([!self.0[0], !self.0[1], !self.0[2], !self.0[3]])
  }
}

impl<Rhs> BitOr<Rhs> for BitBoard
where
  Rhs: AsRef<[u64; 4]>,
{
  type Output = BitBoard;

  fn bitor(self, rhs: Rhs) -> Self::Output {
    let rhs = rhs.as_ref();
    Self([
      self.0[0] | rhs[0],
      self.0[1] | rhs[1],
      self.0[2] | rhs[2],
      self.0[3] | rhs[3],
    ])
  }
}

impl<Rhs> BitOrAssign<Rhs> for BitBoard
where
  Rhs: AsRef<[u64; 4]>,
{
  fn bitor_assign(&mut self, rhs: Rhs) {
    let rhs = rhs.as_ref();
    self.0[0] |= rhs[0];
    self.0[1] |= rhs[1];
    self.0[2] |= rhs[2];
    self.0[3] |= rhs[3];
  }
}

impl<Rhs> BitAndAssign<Rhs> for BitBoard
where
  Rhs: AsRef<[u64; 4]>,
{
  fn bitand_assign(&mut self, rhs: Rhs) {
    let rhs = rhs.as_ref();
    self.0[0] &= rhs[0];
    self.0[1] &= rhs[1];
    self.0[2] &= rhs[2];
    self.0[3] &= rhs[3];
  }
}

impl<Rhs> BitAnd<Rhs> for BitBoard
where
  Rhs: AsRef<[u64; 4]>,
{
  type Output = BitBoard;

  fn bitand(self, rhs: Rhs) -> Self::Output {
    let rhs = rhs.as_ref();
    Self([
      self.0[0] & rhs[0],
      self.0[1] & rhs[1],
      self.0[2] & rhs[2],
      self.0[3] & rhs[3],
    ])
  }
}
