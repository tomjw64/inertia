use core::fmt;
use std::cmp::min;

// use crate::mechanics::ActorSquares;
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
    let mut square_crawls = [HeuristicValue::MAX; 256];
    let mut current_square_set = vec![(Square(goal.0), 0)];
    let mut next_square_set = Vec::new();

    while !current_square_set.is_empty() {
      for &(square, crawls) in current_square_set.iter() {
        let square_index = square.0 as usize;

        if square_crawls[square_index] <= crawls {
          continue;
        }
        square_crawls[square_index] = min(square_crawls[square_index], crawls);

        for direction in Direction::VARIANTS {
          let move_destination =
            board.get_unimpeded_move_destination(square, direction);
          if move_destination == square {
            next_square_set.extend(
              board
                .get_unimpeded_movement_ray_squares(
                  square,
                  direction.opposite(),
                )
                .into_iter()
                .map(|s| (s, crawls)),
            );
          } else {
            next_square_set.push((
              square
                .get_adjacent(direction)
                .expect("Adjacent must exist if movement is unimpeded"),
              crawls + 1,
            ))
          }
        }
      }
      current_square_set = next_square_set;
      next_square_set = Vec::new();
    }

    Self {
      squares: square_crawls,
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
