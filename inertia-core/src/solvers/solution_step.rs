use serde::Deserialize;
use serde::Serialize;
use typeshare::typeshare;

use crate::mechanics::Direction;

#[typeshare]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolutionStep {
  #[typeshare(typescript(type = "number"))]
  pub actor: usize,
  pub direction: Direction,
}

impl From<(usize, Direction)> for SolutionStep {
  fn from(value: (usize, Direction)) -> Self {
    Self {
      actor: value.0,
      direction: value.1,
    }
  }
}
