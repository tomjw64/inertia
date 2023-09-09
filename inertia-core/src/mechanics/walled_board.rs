use core::fmt;

use serde::Deserialize;
use serde::Serialize;
use typeshare::typeshare;

use crate::mechanics::Square;

#[typeshare]
type WallGroup = [bool; 15];
#[typeshare]
type WallGrid = [WallGroup; 16];

#[typeshare]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct WalledBoard {
  pub(crate) vertical: WallGrid, // 16 ROWS of 15
  pub(crate) horizontal: WallGrid, // 16 COLUMNS of 15
                                 // pub goal: Square,
}

impl WalledBoard {
  pub(crate) const EMPTY: Self = Self {
    vertical: [[false; 15]; 16],
    horizontal: [[false; 15]; 16],
    // goal: Square(0),
  };

  pub(crate) fn col(&self, col: usize) -> &WallGroup {
    &self.horizontal[col]
  }

  pub(crate) fn col_mut(&mut self, col: usize) -> &mut WallGroup {
    &mut self.horizontal[col]
  }

  pub(crate) fn row(&self, row: usize) -> &WallGroup {
    &self.vertical[row]
  }

  pub(crate) fn row_mut(&mut self, row: usize) -> &mut WallGroup {
    &mut self.vertical[row]
  }
}

impl fmt::Display for WalledBoard {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str("\n")?;
    f.write_str("\u{2588}".repeat(66).as_str())?;
    for row in 0..16 {
      f.write_str("\n")?;
      f.write_str("\u{2588}".repeat(2).as_str())?;
      for column in 0..15 {
        f.write_str(" ".repeat(2).as_str())?;
        if self.row(row)[column] {
          f.write_str("\u{2588}".repeat(2).as_str())?;
        } else {
          f.write_str("\u{2591}".repeat(2).as_str())?;
        }
      }
      f.write_str(" ".repeat(2).as_str())?;
      f.write_str("\u{2588}".repeat(2).as_str())?;
      if row == 15 {
        continue;
      }
      f.write_str("\n")?;
      f.write_str("\u{2588}".repeat(2).as_str())?;
      for column in 0..16 {
        if self.col(column)[row] {
          f.write_str("\u{2588}".repeat(2).as_str())?;
        } else {
          f.write_str("\u{2591}".repeat(2).as_str())?;
        }
        if column < 15 {
          f.write_str("\u{2591}".repeat(2).as_str())?;
        }
      }
      f.write_str("\u{2588}".repeat(2).as_str())?;
    }
    f.write_str("\n")?;
    f.write_str("\u{2588}".repeat(66).as_str())?;
    Ok(())
  }
}
