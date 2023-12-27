use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;
use typeshare::typeshare;

use crate::mechanics::ActorSquares;
use crate::mechanics::Square;
use crate::mechanics::WalledBoard;
use crate::solvers::SolutionStep;

use super::MoveBoard;

#[typeshare]
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct WalledBoardPosition {
  pub walled_board: WalledBoard,
  pub actor_squares: ActorSquares,
  pub goal: Square,
}

impl Default for WalledBoardPosition {
  fn default() -> Self {
    Self {
      walled_board: WalledBoard::EMPTY,
      goal: Square(255),
      actor_squares: ActorSquares([Square(0), Square(1), Square(2), Square(3)]),
    }
  }
}

impl WalledBoardPosition {
  pub fn to_compressed_byte_array(&self) -> [u8; 69] {
    let mut bytes = [0u8; 69];
    let mut offset = 0;
    // 32 bytes for vertical walls
    for row in self.walled_board.vertical {
      for (idx, &bit) in row.iter().enumerate() {
        let byte = idx / 8;
        let shift = idx % 8;
        bytes[offset + byte] |= (bit as u8) << shift;
      }
      offset += 2;
    }
    // 32 bytes for horizontal walls
    for col in self.walled_board.horizontal {
      for (idx, &bit) in col.iter().enumerate() {
        let byte = idx / 8;
        let shift = idx % 8;
        bytes[offset + byte] |= (bit as u8) << shift;
      }
      offset += 2;
    }
    // 4 bytes for actor squares
    self.actor_squares.as_bytes().into_iter().for_each(|byte| {
      bytes[offset] = byte;
      offset += 1;
    });
    // 1 byte for goal
    bytes[offset] = self.goal.0;
    // All done
    bytes
  }

  pub fn from_compressed_byte_array(bytes: &[u8; 69]) -> Self {
    let mut vertical = [[false; 15]; 16];
    for (byte_idx, byte) in bytes.iter().enumerate().take(32) {
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
    for (byte_idx, byte) in bytes.iter().enumerate().take(64).skip(32) {
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
      bytes[64..69]
        .split_at(std::mem::size_of::<u32>())
        .0
        .try_into()
        .expect("This is exactly 4 bytes"),
    );
    let goal = Square(bytes[68]);
    Self {
      walled_board: WalledBoard {
        vertical,
        horizontal,
      },
      actor_squares,
      goal,
    }
  }

  pub fn is_solution(&self, solution: &[SolutionStep]) -> bool {
    let actor_squares = self.apply_solution(solution);

    actor_squares.0.contains(&self.goal)
  }

  pub fn apply_solution(&self, solution: &[SolutionStep]) -> ActorSquares {
    let move_board = MoveBoard::from(&self.walled_board);
    let mut actor_squares = self.actor_squares;
    for &SolutionStep {
      actor: actor_index,
      direction,
    } in solution
    {
      let actor_square = actor_squares.0[actor_index as usize];
      let move_destination =
        move_board.get_move_destination(actor_square, actor_squares, direction);
      actor_squares.0[actor_index as usize] = move_destination;
    }
    actor_squares
  }
}

pub trait DynCloneWalledBoardPositionGenerator {
  fn clone_dyn(&self) -> Box<dyn WalledBoardPositionGenerator>;
}

impl<T> DynCloneWalledBoardPositionGenerator for T
where
  T: 'static + WalledBoardPositionGenerator + Clone,
{
  fn clone_dyn(&self) -> Box<dyn WalledBoardPositionGenerator> {
    Box::new(self.clone())
  }
}

pub trait WalledBoardPositionGenerator:
  DynCloneWalledBoardPositionGenerator + std::fmt::Debug + Send + Sync
{
  fn generate_position(&self) -> WalledBoardPosition;
}

impl Clone for Box<dyn WalledBoardPositionGenerator> {
  fn clone(&self) -> Self {
    self.clone_dyn()
  }
}

pub struct SolvedPosition {
  pub position: WalledBoardPosition,
  pub solution: Vec<SolutionStep>,
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

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn to_and_from_bytes_identity_empty() {
    let position = WalledBoardPosition {
      walled_board: WalledBoard::EMPTY,
      actor_squares: ActorSquares([Square(0); 4]),
      goal: Square(255),
    };
    let identity = WalledBoardPosition::from_compressed_byte_array(
      &position.to_compressed_byte_array(),
    );
    assert_eq!(position, identity)
  }

  #[test]
  fn to_and_from_bytes_identity_generated() {
    let position = WalledBoardPosition {
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

    let identity = WalledBoardPosition::from_compressed_byte_array(
      &position.to_compressed_byte_array(),
    );
    assert_eq!(position, identity)
  }
}
