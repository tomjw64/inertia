use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use serde::Deserialize;
use serde::Serialize;

#[cfg(feature = "web")]
use {tsify::Tsify, wasm_bindgen::prelude::wasm_bindgen};

#[derive(
  Copy,
  Clone,
  PartialEq,
  Eq,
  Debug,
  Serialize,
  Deserialize,
  IntoPrimitive,
  TryFromPrimitive,
)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
#[repr(u8)]
pub enum Direction {
  Up = 0,
  Down = 1,
  Left = 2,
  Right = 3,
}

impl Direction {
  pub const VARIANTS: [Self; 4] =
    [Self::Up, Self::Down, Self::Left, Self::Right];

  pub const fn opposite(self) -> Self {
    match self {
      Direction::Up => Direction::Down,
      Direction::Down => Direction::Up,
      Direction::Left => Direction::Right,
      Direction::Right => Direction::Left,
    }
  }
}
