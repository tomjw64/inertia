use core::fmt;
use std::collections::VecDeque;

use crate::mechanics::ActorSquares;
use crate::mechanics::Direction;
use crate::mechanics::MoveBoard;
use crate::mechanics::Square;
use crate::solvers::format_square_heuristics;

use super::get_min;
use super::Heuristic;
use super::HeuristicValue;

pub struct MinAssistsBoard {
  pub squares: [HeuristicValue; 256],
}

impl fmt::Debug for MinAssistsBoard {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    format_square_heuristics(f, &self.squares)
  }
}

impl MinAssistsBoard {
  pub fn get(&self, square: Square) -> HeuristicValue {
    self.squares[square.0 as usize]
  }

  pub fn from_move_board(board: &MoveBoard, goal: Square) -> Self {
    let mut min_assists_board = [HeuristicValue::MAX; 256];
    let mut queue = VecDeque::new();
    queue.push_back((Square(goal.0), 0));

    while let Some((square, min_assists)) = queue.pop_front() {
      let square_index = square.0 as usize;

      if min_assists_board[square_index] <= min_assists {
        continue;
      }
      min_assists_board[square_index] = min_assists;

      for direction in Direction::VARIANTS {
        let move_destination =
          board.get_unimpeded_move_destination(square, direction);
        if move_destination == square {
          queue.extend(
            board
              .get_unimpeded_movement_ray_squares(square, direction.opposite())
              .into_iter()
              .map(|s| (s, min_assists)),
          );
        } else {
          queue.extend(
            board
              .get_unimpeded_movement_ray_squares(square, direction)
              .into_iter()
              .map(|s| (s, min_assists + 1)),
          );
        }
      }
    }

    Self {
      squares: min_assists_board,
    }
  }
}

impl Heuristic for MinAssistsBoard {
  // This is an admissible heuristic because because the value for each square
  // represents the minimum number of assists it would require for an actor on
  // that square to move to the goal square. An assist can not occur without
  // moving. Thus, this is a lower bound for the number of moves to complete a
  // puzzle.
  fn get_heuristic(&self, actor_squares: ActorSquares) -> HeuristicValue {
    get_min(
      actor_squares
        .0
        .map(|square| self.squares[square.0 as usize]),
    )
  }

  fn get_heuristic_for_target_actor(
    &self,
    actor_squares: ActorSquares,
    actor_index: usize,
  ) -> HeuristicValue {
    self.squares[actor_squares.0[actor_index].0 as usize]
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::mechanics::B64EncodedCompressedPosition;
  use crate::mechanics::CompressedPosition;
  use crate::mechanics::MoveBoard;
  use crate::mechanics::Position;

  #[test]
  fn test_sample_position_heuristics() {
    for &(name, position_b64, _) in inertia_fixtures::SAMPLE_POSITIONS {
      let position = Position::try_from(
        CompressedPosition::try_from(B64EncodedCompressedPosition(
          position_b64.to_owned(),
        ))
        .unwrap(),
      )
      .unwrap();
      let board = MinAssistsBoard::from_move_board(
        &MoveBoard::from(&position.walled_board),
        position.goal,
      );
      insta::assert_debug_snapshot!(name, board);
    }
  }
}
