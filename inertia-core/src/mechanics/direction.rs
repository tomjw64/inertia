use serde::Deserialize;
use serde::Serialize;
use typeshare::typeshare;

#[typeshare]
#[derive(Copy, Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
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
