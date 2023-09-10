use std::cmp::min;

use serde::Deserialize;
use serde::Serialize;
use typeshare::typeshare;

#[typeshare]
#[derive(
  Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct Square(pub(crate) u8);

impl Square {
  pub const fn new(index: u8) -> Square {
    Square(index)
  }

  pub fn from_row_col(row: usize, col: usize) -> Self {
    let row = min(row, 15);
    let col = min(col, 15);
    Square((row * 16 + col) as u8)
  }

  pub const fn as_row_col(self) -> (u8, u8) {
    (self.0 / 16, self.0 % 16)
  }

  pub fn get_adjacent_and_self(self) -> Vec<Square> {
    let index = self.0;
    let mut adj = vec![self];
    if index > 15 {
      adj.push(Square(index - 16))
    }
    if index < 240 {
      adj.push(Square(index + 16))
    }
    if index % 16 != 0 {
      adj.push(Square(index - 1))
    }
    if index % 16 != 15 {
      adj.push(Square(index + 1))
    }
    adj
  }

  pub fn get_adjacent(self) -> Vec<Square> {
    let index = self.0;
    let mut adj = vec![];
    if index > 15 {
      adj.push(Square(index - 16))
    }
    if index < 240 {
      adj.push(Square(index + 16))
    }
    if index % 16 != 0 {
      adj.push(Square(index - 1))
    }
    if index % 16 != 15 {
      adj.push(Square(index + 1))
    }
    adj
  }
}
