use super::SolutionStep;
use itertools::Itertools;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
pub enum Difficulty {
  Easiest = 0,
  Easy = 1,
  Medium = 2,
  Hard = 3,
  Hardest = 4,
}

impl From<Difficulty> for u8 {
  fn from(value: Difficulty) -> Self {
    value as u8
  }
}

impl TryFrom<u8> for Difficulty {
  type Error = ();
  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      0 => Ok(Difficulty::Easiest),
      1 => Ok(Difficulty::Easy),
      2 => Ok(Difficulty::Medium),
      3 => Ok(Difficulty::Hard),
      4 => Ok(Difficulty::Hardest),
      _ => Err(()),
    }
  }
}

pub fn get_solution_difficulty(steps: &[SolutionStep]) -> Difficulty {
  let steps_count = steps.len();
  let actor_count = steps.iter().map(|step| step.actor).unique().count();
  let focus_count = steps.iter().map(|step| step.actor).dedup().count();
  match (steps_count, actor_count, focus_count) {
    (0..=4, _, _) | (0..=6, 0..=1, 0..=1) => Difficulty::Easiest,
    (_, 0..=1, 0..=1) | (0..=6, 0..=2, 0..=2) => Difficulty::Easy,
    (0..=8, 0..=4, 0..=5) => Difficulty::Medium,
    (0..=12, 0..=4, 0..=8) => Difficulty::Hard,
    _ => Difficulty::Hardest,
  }
}
