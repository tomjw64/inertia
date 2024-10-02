use super::Square;
use core::fmt;
use serde::Deserialize;
use serde::Serialize;

#[cfg(feature = "web")]
use {tsify::declare, tsify::Tsify, wasm_bindgen::prelude::wasm_bindgen};

#[cfg_attr(feature = "web", declare)]
type WallGroup = [bool; 15];
#[cfg_attr(feature = "web", declare)]
type WallGrid = [WallGroup; 16];

#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct Walls {
  pub(crate) up: bool,
  pub(crate) down: bool,
  pub(crate) left: bool,
  pub(crate) right: bool,
}

impl Walls {
  pub(crate) fn is_corner(&self) -> bool {
    self.up as u8 + self.down as u8 == 1
      && self.left as u8 + self.right as u8 == 1
  }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct WalledBoard {
  pub vertical: WallGrid,   // 16 ROWS of 15
  pub horizontal: WallGrid, // 16 COLUMNS of 15
}

impl WalledBoard {
  pub const EMPTY: Self = Self {
    vertical: [[false; 15]; 16],
    horizontal: [[false; 15]; 16],
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

  pub(crate) fn set_wall_up<T: Into<Square>>(
    &mut self,
    square: T,
    value: bool,
  ) {
    let (row, col) = square.into().as_row_col();
    if row == 0 {
      return;
    }
    self.col_mut(col)[row - 1] = value;
  }

  pub(crate) fn set_wall_down<T: Into<Square>>(
    &mut self,
    square: T,
    value: bool,
  ) {
    let (row, col) = square.into().as_row_col();
    if row == 15 {
      return;
    }
    self.col_mut(col)[row] = value;
  }

  pub(crate) fn set_wall_left<T: Into<Square>>(
    &mut self,
    square: T,
    value: bool,
  ) {
    let (row, col) = square.into().as_row_col();
    if col == 0 {
      return;
    }
    self.row_mut(row)[col - 1] = value;
  }

  pub(crate) fn set_wall_right<T: Into<Square>>(
    &mut self,
    square: T,
    value: bool,
  ) {
    let (row, col) = square.into().as_row_col();
    if col == 15 {
      return;
    }
    self.row_mut(row)[col] = value;
  }

  pub(crate) fn walls_for_square<T: Into<Square>>(
    &self,
    square: T,
    allow_edges: bool,
  ) -> Walls {
    let square = square.into();

    let up = self.get_wall_up(square, allow_edges);
    let down = self.get_wall_down(square, allow_edges);
    let left = self.get_wall_left(square, allow_edges);
    let right = self.get_wall_right(square, allow_edges);

    Walls {
      up,
      down,
      left,
      right,
    }
  }

  pub(crate) fn get_wall_up<T: Into<Square>>(
    &self,
    square: T,
    allow_edges: bool,
  ) -> bool {
    let (row, col) = square.into().as_row_col();
    if row == 0 {
      return allow_edges;
    }
    self.col(col)[row - 1]
  }

  pub(crate) fn get_wall_down<T: Into<Square>>(
    &self,
    square: T,
    allow_edges: bool,
  ) -> bool {
    let (row, col) = square.into().as_row_col();
    if row == 15 {
      return allow_edges;
    }
    self.col(col)[row]
  }

  pub(crate) fn get_wall_left<T: Into<Square>>(
    &self,
    square: T,
    allow_edges: bool,
  ) -> bool {
    let (row, col) = square.into().as_row_col();
    if col == 0 {
      return allow_edges;
    }
    self.row(row)[col - 1]
  }

  pub(crate) fn get_wall_right<T: Into<Square>>(
    &self,
    square: T,
    allow_edges: bool,
  ) -> bool {
    let (row, col) = square.into().as_row_col();
    if col == 15 {
      return allow_edges;
    }
    self.row(row)[col]
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
