use core::fmt;
use std::cmp::min;
use std::collections::VecDeque;

use crate::mechanics::ActorSquares;
use crate::mechanics::Direction;
use crate::mechanics::MoveBoard;
use crate::mechanics::Square;
use crate::solvers::format_square_heuristics;

use super::get_min;
use super::Heuristic;
use super::HeuristicValue;

pub struct GroupMinMovesExpensiveCrawlsBoard {
  pub squares: [HeuristicValue; 256],
}

impl fmt::Debug for GroupMinMovesExpensiveCrawlsBoard {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    format_square_heuristics(f, &self.squares)
  }
}

impl GroupMinMovesExpensiveCrawlsBoard {
  pub fn get(&self, square: Square) -> HeuristicValue {
    self.squares[square.0 as usize]
  }

  pub fn from_move_board(board: &MoveBoard, goal: Square) -> Self {
    let mut group_min_moves_board = [HeuristicValue::MAX; 256];
    let mut subsequent_crawls_board = [HeuristicValue::MAX; 256];
    let mut queue = VecDeque::new();
    queue.push_back((Square(goal.0), 0, 0));

    while let Some((square, value, subsequent_crawls)) = queue.pop_front() {
      let square_index = square.0 as usize;

      if group_min_moves_board[square_index] < value {
        continue;
      }

      group_min_moves_board[square_index] = value;
      subsequent_crawls_board[square_index] =
        min(subsequent_crawls_board[square_index], subsequent_crawls);

      for direction in Direction::VARIANTS {
        let move_destination =
          board.get_unimpeded_move_destination(square, direction);
        if move_destination == square {
          queue.extend(
            board
              .get_unimpeded_movement_ray_squares(square, direction.opposite())
              .into_iter()
              .map(|s| (s, value + 1, 0)),
          );
        } else {
          queue.push_back((
            square
              .get_adjacent(direction)
              .expect("Adjacent must exist if movement is unimpeded"),
            value + 1 + if subsequent_crawls > 3 { 1 } else { 0 },
            subsequent_crawls + 1,
          ));
        }
      }
    }

    Self {
      squares: group_min_moves_board,
    }
  }
}

impl Heuristic for GroupMinMovesExpensiveCrawlsBoard {
  fn get_heuristic(&self, actor_squares: ActorSquares) -> HeuristicValue {
    get_min(
      actor_squares
        .0
        .map(|square| self.squares[square.0 as usize]),
    )
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
      let board = GroupMinMovesExpensiveCrawlsBoard::from_move_board(
        &MoveBoard::from(&position.walled_board),
        position.goal,
      );
      insta::assert_debug_snapshot!(name, board);
    }
  }
}
