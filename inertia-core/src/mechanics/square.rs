use std::cmp::min;

use serde::Deserialize;
use serde::Serialize;
use typeshare::typeshare;

use super::Direction;

#[typeshare]
#[derive(
  Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize,
)]
pub struct Square(pub u8);

impl From<u8> for Square {
  fn from(value: u8) -> Self {
    Square(value)
  }
}

impl From<Square> for u8 {
  fn from(value: Square) -> Self {
    value.0
  }
}

impl From<(usize, usize)> for Square {
  fn from(value: (usize, usize)) -> Self {
    Self::from_row_col(value.0, value.1)
  }
}

impl Square {
  pub fn from_row_col(row: usize, col: usize) -> Self {
    let row = min(row, 15);
    let col = min(col, 15);
    Square((row * 16 + col) as u8)
  }

  pub const fn as_row_col(self) -> (usize, usize) {
    ((self.0 / 16) as usize, (self.0 % 16) as usize)
  }

  pub fn get_all_adjacent_and_self(self) -> Vec<Square> {
    let mut adj = self.get_all_adjacent();
    adj.push(self);
    adj
  }

  pub fn get_all_adjacent(self) -> Vec<Square> {
    let mut adj = vec![];
    for direction in Direction::VARIANTS {
      if let Some(square) = self.get_adjacent(direction) {
        adj.push(square);
      }
    }
    adj
  }

  pub fn get_adjacent(self, direction: Direction) -> Option<Self> {
    let index = self.0;
    match direction {
      Direction::Up => {
        if index > 15 {
          Some(Square(index - 16))
        } else {
          None
        }
      }
      Direction::Down => {
        if index < 240 {
          Some(Square(index + 16))
        } else {
          None
        }
      }
      Direction::Left => {
        if index % 16 != 0 {
          Some(Square(index - 1))
        } else {
          None
        }
      }
      Direction::Right => {
        if index % 16 != 15 {
          Some(Square(index + 1))
        } else {
          None
        }
      }
    }
  }
}
