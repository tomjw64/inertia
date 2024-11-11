use std::array::TryFromSliceError;

use super::Direction;
use super::MoveBoard;
use crate::mechanics::ActorSquares;
use crate::mechanics::Square;
use crate::mechanics::WalledBoard;
use crate::solvers::Solution;
use crate::solvers::SolutionStep;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::DecodeError;
use base64::Engine;
use num_enum::IntoPrimitive;
use num_enum::TryFromPrimitive;
use num_enum::TryFromPrimitiveError;
use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

const COMPRESSED_POSITION_BYTES_FOR_COMPRESSION_TYPE: usize = 1;

#[cfg(feature = "web")]
use {tsify::Tsify, wasm_bindgen::prelude::wasm_bindgen};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct Position {
  pub walled_board: WalledBoard,
  pub actor_squares: ActorSquares,
  pub goal: Square,
}

impl Default for Position {
  fn default() -> Self {
    Self {
      walled_board: WalledBoard::EMPTY,
      goal: Square(255),
      actor_squares: ActorSquares([Square(0), Square(1), Square(2), Square(3)]),
    }
  }
}

impl Position {
  // TODO: Add version byte to front - add conversion methods

  pub fn is_solution(&self, solution: &Solution) -> bool {
    let actor_squares = self.apply_solution(solution);

    actor_squares.0.contains(&self.goal)
  }

  pub fn apply_solution(&self, solution: &Solution) -> ActorSquares {
    let move_board = MoveBoard::from(&self.walled_board);
    let mut actor_squares = self.actor_squares;
    for &SolutionStep {
      actor: actor_index,
      direction,
    } in &solution.0
    {
      let actor_square = actor_squares.0[actor_index as usize];
      let move_destination =
        move_board.get_move_destination(actor_square, actor_squares, direction);
      actor_squares.0[actor_index as usize] = move_destination;
    }
    actor_squares
  }
}

#[derive(Clone, Copy, Debug)]
pub struct LegacyCompressedPosition(pub [u8; 69]);

#[derive(Clone, Debug)]
pub struct CompressedPosition(pub Vec<u8>);

#[derive(Copy, Clone, Debug, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum PositionCompressionType {
  // TODO: Add alternative compression
  NaiveAssumingAnyActorGoal = 0,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct B64EncodedCompressedPosition(pub String);

#[derive(Error, Debug)]
pub enum PositionConvertError {
  #[error("Failed to parse compression type")]
  CompressionTypeParseError,
  #[error(transparent)]
  CompressionTypeConvertError(
    #[from] TryFromPrimitiveError<PositionCompressionType>,
  ),
  #[error("Failed to decompress with type: {:?}", .0)]
  DecompressionError(PositionCompressionType),
}

#[derive(Error, Debug)]
pub enum CompressedPositionConvertError {
  #[error(transparent)]
  TryFromSliceError(#[from] TryFromSliceError),
  #[error(transparent)]
  DecodeError(#[from] DecodeError),
}

impl TryFrom<Vec<u8>> for LegacyCompressedPosition {
  type Error = CompressedPositionConvertError;

  fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
    Self::try_from(value.as_slice())
  }
}

impl TryFrom<&[u8]> for LegacyCompressedPosition {
  type Error = CompressedPositionConvertError;

  fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
    Ok(Self(<[u8; 69]>::try_from(value)?))
  }
}

impl From<LegacyCompressedPosition> for CompressedPosition {
  fn from(value: LegacyCompressedPosition) -> Self {
    let mut inner =
      vec![PositionCompressionType::NaiveAssumingAnyActorGoal as u8];
    inner.extend_from_slice(&value.0);
    CompressedPosition(inner)
  }
}

impl TryFrom<B64EncodedCompressedPosition> for LegacyCompressedPosition {
  type Error = CompressedPositionConvertError;

  fn try_from(
    value: B64EncodedCompressedPosition,
  ) -> Result<Self, Self::Error> {
    Self::try_from(&value)
  }
}

impl TryFrom<&B64EncodedCompressedPosition> for LegacyCompressedPosition {
  type Error = CompressedPositionConvertError;

  fn try_from(
    value: &B64EncodedCompressedPosition,
  ) -> Result<Self, Self::Error> {
    let bytes = URL_SAFE_NO_PAD.decode(&value.0)?;
    Ok(Self(*&bytes[0..69].try_into()?))
  }
}

impl From<Position> for CompressedPosition {
  fn from(value: Position) -> Self {
    Self::from(&value)
  }
}

impl From<&Position> for CompressedPosition {
  fn from(value: &Position) -> Self {
    let mut bytes = [0u8; 70];
    let mut offset = 0;
    // TODO: Add alternative compression
    bytes[offset] = PositionCompressionType::NaiveAssumingAnyActorGoal as u8;
    offset += 1;
    // 32 bytes for vertical walls
    for row in value.walled_board.vertical {
      for (idx, &bit) in row.iter().enumerate() {
        let byte = idx / 8;
        let shift = idx % 8;
        bytes[offset + byte] |= (bit as u8) << shift;
      }
      offset += 2;
    }
    // 32 bytes for horizontal walls
    for col in value.walled_board.horizontal {
      for (idx, &bit) in col.iter().enumerate() {
        let byte = idx / 8;
        let shift = idx % 8;
        bytes[offset + byte] |= (bit as u8) << shift;
      }
      offset += 2;
    }
    // 4 bytes for actor squares
    value.actor_squares.as_bytes().into_iter().for_each(|byte| {
      bytes[offset] = byte;
      offset += 1;
    });
    // 1 byte for goal square
    bytes[offset] = value.goal.0;
    // All done
    Self(Vec::from(bytes))
  }
}

impl TryFrom<CompressedPosition> for Position {
  type Error = PositionConvertError;

  fn try_from(value: CompressedPosition) -> Result<Self, Self::Error> {
    Self::try_from(&value)
  }
}

impl TryFrom<&CompressedPosition> for Position {
  type Error = PositionConvertError;

  fn try_from(value: &CompressedPosition) -> Result<Self, Self::Error> {
    let bytes = &value.0;
    let (compression_byte, position_bytes) = bytes
      .split_at_checked(COMPRESSED_POSITION_BYTES_FOR_COMPRESSION_TYPE)
      .ok_or(PositionConvertError::CompressionTypeParseError)?;
    let compression_type =
      PositionCompressionType::try_from(compression_byte[0])?;
    match compression_type {
      PositionCompressionType::NaiveAssumingAnyActorGoal => {
        if position_bytes.len() != 69 {
          return Err(PositionConvertError::DecompressionError(
            compression_type,
          ));
        }
        let mut vertical = [[false; 15]; 16];
        for (byte_idx, byte) in position_bytes.iter().enumerate().take(32) {
          for bit_idx in 0..8 {
            let row_idx = byte_idx / 2;
            let bool_idx = bit_idx + ((byte_idx % 2) * 8);
            if bool_idx >= 15 {
              continue;
            }
            vertical[row_idx][bool_idx] = byte & (1 << bit_idx) > 0;
          }
        }
        let mut horizontal = [[false; 15]; 16];
        for (byte_idx, byte) in
          position_bytes.iter().enumerate().take(64).skip(32)
        {
          for bit_idx in 0..8 {
            let col_idx = (byte_idx - 32) / 2;
            let bool_idx = bit_idx + ((byte_idx % 2) * 8);
            if bool_idx >= 15 {
              continue;
            }
            horizontal[col_idx][bool_idx] = byte & (1 << bit_idx) > 0;
          }
        }
        let actor_squares = ActorSquares::from_bytes(
          position_bytes[64..69]
            .split_at(std::mem::size_of::<u32>())
            .0
            .try_into()
            .expect("This is exactly 4 bytes"),
        );
        let goal = Square(position_bytes[68]);
        Ok(Self {
          walled_board: WalledBoard {
            vertical,
            horizontal,
          },
          actor_squares,
          goal,
        })
      }
    }
  }
}

impl TryFrom<B64EncodedCompressedPosition> for CompressedPosition {
  type Error = CompressedPositionConvertError;

  fn try_from(
    value: B64EncodedCompressedPosition,
  ) -> Result<Self, Self::Error> {
    Self::try_from(&value)
  }
}

impl TryFrom<&B64EncodedCompressedPosition> for CompressedPosition {
  type Error = CompressedPositionConvertError;

  fn try_from(
    value: &B64EncodedCompressedPosition,
  ) -> Result<Self, Self::Error> {
    let bytes = URL_SAFE_NO_PAD.decode(&value.0)?;
    Ok(Self(bytes))
  }
}

impl From<CompressedPosition> for B64EncodedCompressedPosition {
  fn from(value: CompressedPosition) -> Self {
    Self::from(&value)
  }
}

impl From<&CompressedPosition> for B64EncodedCompressedPosition {
  fn from(value: &CompressedPosition) -> Self {
    Self(URL_SAFE_NO_PAD.encode(&value.0))
  }
}

pub trait CloneDynPositionGenerator {
  fn clone_dyn(&self) -> Box<dyn PositionGenerator>;
}

impl<T> CloneDynPositionGenerator for T
where
  T: 'static + PositionGenerator + Clone,
{
  fn clone_dyn(&self) -> Box<dyn PositionGenerator> {
    Box::new(self.clone())
  }
}

pub trait PositionGenerator:
  CloneDynPositionGenerator + std::fmt::Debug + Send + Sync
{
  fn generate_position(&self) -> Position;
}

impl Clone for Box<dyn PositionGenerator> {
  fn clone(&self) -> Self {
    self.clone_dyn()
  }
}

pub struct SolvedPosition {
  pub position: Position,
  pub solution: Solution,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub enum CheckSolutionResult {
  NotASolution,
  InferiorSolution,
  ComparableSolution,
  SuperiorSolution,
}

impl Default for SolvedPosition {
  fn default() -> Self {
    Self {
      position: Position::default(),
      solution: Solution(vec![
        SolutionStep {
          actor: 0,
          direction: Direction::Down,
        },
        SolutionStep {
          actor: 0,
          direction: Direction::Right,
        },
      ]),
    }
  }
}

impl SolvedPosition {
  pub fn check_solution(&self, solution: &Solution) -> CheckSolutionResult {
    if !self.position.is_solution(solution) {
      return CheckSolutionResult::NotASolution;
    }
    if solution.0.len() > self.solution.0.len() {
      return CheckSolutionResult::InferiorSolution;
    }
    if solution.0.len() < self.solution.0.len() {
      return CheckSolutionResult::SuperiorSolution;
    }
    return CheckSolutionResult::ComparableSolution;
  }
}

pub trait CloneDynSolvedPositionGenerator {
  fn clone_dyn(&self) -> Box<dyn SolvedPositionGenerator>;
}

impl<T> CloneDynSolvedPositionGenerator for T
where
  T: 'static + SolvedPositionGenerator + Clone,
{
  fn clone_dyn(&self) -> Box<dyn SolvedPositionGenerator> {
    Box::new(self.clone())
  }
}

pub trait SolvedPositionGenerator:
  CloneDynSolvedPositionGenerator + std::fmt::Debug + Send + Sync
{
  fn generate_solved_position(&self) -> SolvedPosition;
}

impl Clone for Box<dyn SolvedPositionGenerator> {
  fn clone(&self) -> Self {
    self.clone_dyn()
  }
}

impl<T> PositionGenerator for T
where
  T: 'static + SolvedPositionGenerator + Clone,
{
  fn generate_position(&self) -> Position {
    self.generate_solved_position().position
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn to_and_from_bytes_identity_empty() {
    let position = Position {
      walled_board: WalledBoard::EMPTY,
      actor_squares: ActorSquares([Square(0); 4]),
      goal: Square(255),
    };
    let identity =
      Position::try_from(CompressedPosition::from(position)).unwrap();
    assert_eq!(position, identity)
  }

  #[test]
  fn to_and_from_bytes_identity_generated() {
    let position = Position {
      walled_board: WalledBoard {
        vertical: [
          [
            false, false, false, false, false, false, true, false, false,
            false, false, false, true, false, false,
          ],
          [
            false, false, false, true, false, false, false, false, true, false,
            false, false, false, false, false,
          ],
          [
            false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false,
          ],
          [
            false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false,
          ],
          [
            false, false, false, false, true, false, true, false, false, false,
            false, false, false, false, false,
          ],
          [
            false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false,
          ],
          [
            false, false, false, false, false, false, false, false, false,
            false, false, false, true, true, false,
          ],
          [
            false, false, false, false, false, false, true, false, true, false,
            false, false, false, false, false,
          ],
          [
            false, false, false, false, false, false, true, false, true, false,
            false, false, false, false, false,
          ],
          [
            false, true, false, false, false, false, false, false, false,
            false, false, false, false, false, true,
          ],
          [
            false, false, false, false, false, false, true, false, false,
            false, true, false, false, false, false,
          ],
          [
            false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false,
          ],
          [
            false, false, false, false, false, false, true, false, false,
            false, false, false, false, false, false,
          ],
          [
            false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false,
          ],
          [
            false, false, false, false, false, false, false, false, false,
            false, false, false, false, true, false,
          ],
          [
            false, false, true, false, false, false, false, false, false,
            false, false, false, true, false, false,
          ],
        ],
        horizontal: [
          [
            false, true, false, false, false, false, false, false, true, false,
            false, false, false, false, false,
          ],
          [
            false, false, false, false, false, false, false, false, false,
            true, false, false, false, false, false,
          ],
          [
            false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false,
          ],
          [
            false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false,
          ],
          [
            true, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false,
          ],
          [
            false, false, false, false, true, false, false, false, false,
            false, false, false, false, false, false,
          ],
          [
            false, false, false, true, false, false, false, false, false,
            false, true, true, false, false, false,
          ],
          [
            false, false, false, false, false, false, true, false, true, false,
            false, false, false, false, false,
          ],
          [
            false, true, false, false, false, false, true, false, true, false,
            false, false, false, false, false,
          ],
          [
            false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false,
          ],
          [
            false, false, false, false, false, false, false, false, false,
            true, false, false, false, false, false,
          ],
          [
            false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false,
          ],
          [
            false, false, false, false, false, true, false, false, false,
            false, false, false, false, false, false,
          ],
          [
            false, false, false, false, false, false, false, false, false,
            false, false, false, false, true, false,
          ],
          [
            false, false, false, false, false, false, true, false, true, false,
            false, false, false, false, false,
          ],
          [
            false, false, false, false, true, false, false, false, false,
            false, true, false, false, false, false,
          ],
        ],
      },
      actor_squares: ActorSquares([37, 108, 57, 50].map(Square)),
      goal: Square(184),
    };

    let identity =
      Position::try_from(CompressedPosition::from(position)).unwrap();
    assert_eq!(position, identity)
  }
}
