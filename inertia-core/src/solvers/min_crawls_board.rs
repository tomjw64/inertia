use core::fmt;
use std::collections::VecDeque;

use crate::mechanics::Direction;
use crate::mechanics::MoveBoard;
use crate::mechanics::Square;
use crate::solvers::format_square_heuristics;

use super::HeuristicValue;

pub struct MinCrawlsBoard {
  pub squares: [HeuristicValue; 256],
}

impl fmt::Debug for MinCrawlsBoard {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    format_square_heuristics(f, &self.squares)
  }
}

impl MinCrawlsBoard {
  pub fn get(&self, square: Square) -> HeuristicValue {
    self.squares[square.0 as usize]
  }

  pub fn from_move_board(board: &MoveBoard, goal: Square) -> Self {
    let mut min_crawls_board = [HeuristicValue::MAX; 256];
    let mut queue = VecDeque::new();
    queue.push_back((Square(goal.0), 0));

    while let Some((square, min_crawls)) = queue.pop_front() {
      let square_index = square.0 as usize;

      if min_crawls_board[square_index] <= min_crawls {
        continue;
      }
      min_crawls_board[square_index] = min_crawls;

      for direction in Direction::VARIANTS {
        let move_destination =
          board.get_unimpeded_move_destination(square, direction);
        if move_destination == square {
          queue.extend(
            board
              .get_unimpeded_movement_ray_squares(square, direction.opposite())
              .into_iter()
              .map(|s| (s, min_crawls)),
          );
        } else {
          queue.push_back((
            square
              .get_adjacent(direction)
              .expect("Adjacent must exist if movement is unimpeded"),
            min_crawls + 1,
          ))
        }
      }
    }

    Self {
      squares: min_crawls_board,
    }
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
      let board = MinCrawlsBoard::from_move_board(
        &MoveBoard::from(&position.walled_board),
        position.goal,
      );
      insta::assert_debug_snapshot!(name, board);
    }
  }
}
