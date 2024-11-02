use super::SolutionStep;
use itertools::Itertools;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use serde::Deserialize;
use serde::Serialize;

#[cfg(feature = "web")]
use {tsify::Tsify, wasm_bindgen::prelude::wasm_bindgen};

#[derive(
  Debug,
  PartialEq,
  Eq,
  Hash,
  Copy,
  Clone,
  PartialOrd,
  Ord,
  Deserialize,
  Serialize,
)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
#[repr(u8)]
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

impl Difficulty {
  pub fn from_internal_difficulty(value: usize) -> Self {
    match value {
      0..=1 => Difficulty::Easiest,
      2..=3 => Difficulty::Easy,
      4..=6 => Difficulty::Medium,
      7..=9 => Difficulty::Hard,
      _ => Difficulty::Hardest,
    }
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

impl Distribution<Difficulty> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Difficulty {
    Difficulty::try_from(rng.gen_range(0..=4))
      .expect("known range of difficulty values")
  }
}

pub fn get_solution_difficulty(steps: &[SolutionStep]) -> Difficulty {
  Difficulty::from_internal_difficulty(get_solution_internal_difficulty(steps))
}

pub fn get_solution_internal_difficulty(steps: &[SolutionStep]) -> usize {
  let steps_count: usize = steps.len();
  let actor_count = steps.iter().map(|step| step.actor).unique().count();
  let focus_switch_count = steps.iter().map(|step| step.actor).dedup().count();
  let result = match (steps_count, actor_count, focus_switch_count) {
    (1..=2, _, _) => 0,
    (3..=4, 0..=1, 0..=1) => 1,
    (3..=4, _, _) => 2,
    (5..=7, 0..=2, 0..=2) => 3,
    (5..=7, _, _) => 4,
    (8..=9, 0..=2, 0..=2) => 5,
    (_, 1, 1) | (8..=9, _, 3..=4) => 6,
    (8..=9, _, _) | (10..=12, _, 0..=4) => 7,
    (10..=12, _, 0..=6) => 8,
    (10..=12, _, 0..=8) | (13..=15, _, 0..=6) => 9,
    (10..=12, _, 0..=10) | (13..=15, _, 0..=8) | (16..=18, _, 0..=6) => 10,
    _ => 11,
  };
  result
}
