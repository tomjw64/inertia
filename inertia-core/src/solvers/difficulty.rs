use super::Solution;
use itertools::Itertools;
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
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
  IntoPrimitive,
  TryFromPrimitive,
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

impl Distribution<Difficulty> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Difficulty {
    Difficulty::try_from(rng.gen_range(0..=4))
      .expect("known range of difficulty values")
  }
}

pub fn get_solution_difficulty(solution: &Solution) -> Difficulty {
  Difficulty::from_internal_difficulty(get_solution_internal_difficulty(
    solution,
  ))
}

pub fn get_solution_internal_difficulty(solution: &Solution) -> usize {
  let steps = &solution.0;
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
