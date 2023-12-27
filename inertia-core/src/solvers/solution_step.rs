use serde::Deserialize;
use serde::Serialize;
use typeshare::typeshare;

use crate::mechanics::Direction;

#[typeshare]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SolutionStep {
  #[typeshare(typescript(type = "number"))]
  pub actor: u8,
  pub direction: Direction,
}

impl From<(u8, Direction)> for SolutionStep {
  fn from(value: (u8, Direction)) -> Self {
    Self {
      actor: value.0,
      direction: value.1,
    }
  }
}

pub fn solution_to_bytes(solution: &[SolutionStep]) -> Vec<u8> {
  let bytes_for_length = 2;
  let mut bytes =
    vec![0u8; bytes_for_length + solution.len() / 2 + solution.len() % 2];
  let length = (solution.len() as u16).to_le_bytes();
  let (length_bytes, solution_bytes) = bytes.split_at_mut(bytes_for_length);
  length_bytes.copy_from_slice(&length);
  // 2 steps per byte, assumes max 4 actors
  for (step_index, step) in solution.iter().enumerate() {
    let byte_index = step_index / 2;
    let nibble_shift = (step_index % 2) * 4;
    let step_nibble = (step.actor << 2 | step.direction as u8) & 0b1111;
    solution_bytes[byte_index] |= step_nibble << nibble_shift;
  }
  bytes
}

pub fn solution_from_bytes(bytes: &[u8]) -> Result<Vec<SolutionStep>, ()> {
  let bytes_for_length = 2;
  let (length_bytes, solution_bytes) = bytes.split_at(bytes_for_length);
  let length =
    u16::from_le_bytes(length_bytes.try_into().or(Err(()))?) as usize;
  let mut steps = Vec::with_capacity(length);
  // Simple 1 byte per step
  for byte in solution_bytes {
    let first_step_nibble = byte & 0b1111;
    let second_step_nibble = byte >> 4;

    if steps.len() < length {
      steps.push(SolutionStep {
        actor: first_step_nibble >> 2,
        direction: Direction::try_from(first_step_nibble & 0b11)?,
      });
    }

    if steps.len() < length {
      steps.push(SolutionStep {
        actor: second_step_nibble >> 2,
        direction: Direction::try_from(second_step_nibble & 0b11)?,
      });
    }
  }
  Ok(steps)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn to_and_from_bytes_identity_empty() {
    let solution = vec![];
    let identity = solution_from_bytes(&solution_to_bytes(&solution)).unwrap();
    assert_eq!(solution, identity);
  }

  #[test]
  fn to_and_from_bytes_identity_one() {
    let mut solution = vec![SolutionStep {
      actor: 1,
      direction: Direction::Left,
    }];
    let identity = solution_from_bytes(&solution_to_bytes(&solution)).unwrap();
    assert_eq!(solution, identity);
  }

  #[test]
  fn to_and_from_bytes_identity_all() {
    let mut solution = vec![];
    for actor in 0..4u8 {
      for direction in Direction::VARIANTS {
        solution.push(SolutionStep { actor, direction });
      }
    }
    let identity = solution_from_bytes(&solution_to_bytes(&solution)).unwrap();
    assert_eq!(solution, identity);
  }
}
