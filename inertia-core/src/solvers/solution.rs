use crate::mechanics::Direction;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::DecodeError;
use base64::Engine;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

const COMPRESSED_SOLUTION_BYTES_FOR_LENGTH: usize = 2;

#[cfg(feature = "web")]
use {tsify::Tsify, wasm_bindgen::prelude::wasm_bindgen};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct Solution(pub Vec<SolutionStep>);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct CompressedSolution(pub Vec<u8>);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct B64EncodedCompressedSolution(pub String);

#[derive(Error, Debug)]
pub enum SolutionConvertError {
  #[error("Failed to parse solution length")]
  LengthParseError,
}

#[derive(Error, Debug)]
pub enum CompressedSolutionConvertError {
  #[error(transparent)]
  DecodeError(#[from] DecodeError),
}

impl From<Solution> for CompressedSolution {
  fn from(value: Solution) -> Self {
    Self::from(&value)
  }
}

impl From<&Solution> for CompressedSolution {
  fn from(value: &Solution) -> Self {
    let steps = &value.0;
    let mut bytes = vec![
      0u8;
      COMPRESSED_SOLUTION_BYTES_FOR_LENGTH
        + steps.len() / 2
        + steps.len() % 2
    ];
    let length = (steps.len() as u16).to_le_bytes();
    let (length_bytes, solution_bytes) =
      bytes.split_at_mut(COMPRESSED_SOLUTION_BYTES_FOR_LENGTH);
    length_bytes.copy_from_slice(&length);
    // 2 steps per byte, assumes max 4 actors
    for (step_index, step) in steps.iter().enumerate() {
      let byte_index = step_index / 2;
      let nibble_shift = (step_index % 2) * 4;
      let step_nibble = step.actor << 2 | step.direction as u8;
      solution_bytes[byte_index] |= step_nibble << nibble_shift;
    }
    Self(bytes)
  }
}

impl TryFrom<CompressedSolution> for Solution {
  type Error = SolutionConvertError;

  fn try_from(value: CompressedSolution) -> Result<Self, Self::Error> {
    Self::try_from(&value)
  }
}

impl TryFrom<&CompressedSolution> for Solution {
  type Error = SolutionConvertError;

  fn try_from(value: &CompressedSolution) -> Result<Self, Self::Error> {
    let bytes = &value.0;
    let (length_bytes, solution_bytes) = bytes
      .split_at_checked(COMPRESSED_SOLUTION_BYTES_FOR_LENGTH)
      .ok_or(SolutionConvertError::LengthParseError)?;
    let length =
      u16::from_le_bytes([length_bytes[0], length_bytes[1]]) as usize;
    let mut steps = Vec::with_capacity(length);
    // Simple 1 byte per step
    for byte in solution_bytes {
      let first_step_nibble = byte & 0b1111;
      let second_step_nibble = byte >> 4;

      if steps.len() < length {
        steps.push(SolutionStep {
          actor: first_step_nibble >> 2,
          direction: Direction::try_from(first_step_nibble & 0b11)
            .expect("All nibble values are valid"),
        });
      }

      if steps.len() < length {
        steps.push(SolutionStep {
          actor: second_step_nibble >> 2,
          direction: Direction::try_from(second_step_nibble & 0b11)
            .expect("All nibble values are valid"),
        });
      }
    }
    Ok(Self(steps))
  }
}

impl TryFrom<B64EncodedCompressedSolution> for CompressedSolution {
  type Error = CompressedSolutionConvertError;

  fn try_from(
    value: B64EncodedCompressedSolution,
  ) -> Result<Self, Self::Error> {
    Self::try_from(&value)
  }
}

impl TryFrom<&B64EncodedCompressedSolution> for CompressedSolution {
  type Error = CompressedSolutionConvertError;

  fn try_from(
    value: &B64EncodedCompressedSolution,
  ) -> Result<Self, Self::Error> {
    let bytes = URL_SAFE_NO_PAD.decode(&value.0)?;
    Ok(Self(bytes))
  }
}

impl From<CompressedSolution> for B64EncodedCompressedSolution {
  fn from(value: CompressedSolution) -> Self {
    Self::from(&value)
  }
}

impl From<&CompressedSolution> for B64EncodedCompressedSolution {
  fn from(value: &CompressedSolution) -> Self {
    Self(URL_SAFE_NO_PAD.encode(&value.0))
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct SolutionStep {
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

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn to_and_from_bytes_identity_empty() {
    let solution = Solution(vec![]);
    let identity =
      Solution::try_from(CompressedSolution::from(&solution)).unwrap();
    assert_eq!(solution, identity);
  }

  #[test]
  fn to_and_from_bytes_identity_one() {
    let solution = Solution(vec![SolutionStep {
      actor: 1,
      direction: Direction::Left,
    }]);
    let identity =
      Solution::try_from(CompressedSolution::from(&solution)).unwrap();
    assert_eq!(solution, identity);
  }

  #[test]
  fn to_and_from_bytes_identity_all() {
    let mut steps = vec![];
    for actor in 0..4u8 {
      for direction in Direction::VARIANTS {
        steps.push(SolutionStep { actor, direction });
      }
    }
    let solution = Solution(steps);
    let identity =
      Solution::try_from(CompressedSolution::from(&solution)).unwrap();
    assert_eq!(solution, identity);
  }
}
