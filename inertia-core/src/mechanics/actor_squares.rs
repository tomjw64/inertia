use serde::Deserialize;
use serde::Serialize;
use typeshare::typeshare;

use crate::mechanics::BitBoard;
use crate::mechanics::Square;

#[typeshare]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(align(4))] // Pretty much always accessed together
pub struct ActorSquares(pub [Square; 4]);

impl ActorSquares {
  pub fn as_bitboard(self) -> BitBoard {
    let mut bitboard = BitBoard::ZERO;
    for Square(index) in self.0 {
      bitboard.set_bit(index as usize);
    }
    bitboard
  }

  pub fn from_bytes(bytes: [u8; 4]) -> Self {
    Self(bytes.map(Square))
  }

  pub fn as_bytes(self) -> [u8; 4] {
    self.0.map(|square| square.0)
  }

  pub fn as_u32(self) -> u32 {
    u32::from_le_bytes(self.0.map(|square| square.0))
  }

  pub fn as_sorted(self) -> ActorSquares {
    let mut bytes = self.0.map(|s| s.0);
    // Optimal sorting network for 4 elements
    if bytes[0] > bytes[1] {
      bytes.swap(0, 1)
    }
    if bytes[2] > bytes[3] {
      bytes.swap(2, 3)
    }
    if bytes[0] > bytes[2] {
      bytes.swap(0, 2)
    }
    if bytes[1] > bytes[3] {
      bytes.swap(1, 3)
    }
    if bytes[1] > bytes[2] {
      bytes.swap(1, 2)
    }
    Self(bytes.map(Square))
  }

  // Optimal sorting network for 4 elements
  pub fn as_sorted_u32(self) -> u32 {
    let mut bytes = self.0.map(|s| s.0);
    if bytes[0] > bytes[1] {
      bytes.swap(0, 1)
    }
    if bytes[2] > bytes[3] {
      bytes.swap(2, 3)
    }
    if bytes[0] > bytes[2] {
      bytes.swap(0, 2)
    }
    if bytes[1] > bytes[3] {
      bytes.swap(1, 3)
    }
    if bytes[1] > bytes[2] {
      bytes.swap(1, 2)
    }
    u32::from_le_bytes(bytes)
  }
}
