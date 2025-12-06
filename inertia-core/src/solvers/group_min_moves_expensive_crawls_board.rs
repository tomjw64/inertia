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

const CONSECUTIVE_CRAWL_THRESHOLD: HeuristicValue = 3;
const PENALTY: HeuristicValue = 1;

pub struct GroupMinMovesExpensiveCrawlsBoard {
  pub squares: [HeuristicValue; 256],
}

impl fmt::Debug for GroupMinMovesExpensiveCrawlsBoard {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    format_square_heuristics(f, &self.squares)
  }
}

fn calc_value_after_threshold_moves(
  value: HeuristicValue,
  consecutive_crawls: HeuristicValue,
) -> HeuristicValue {
  let remaining_crawls_under_threshold =
    CONSECUTIVE_CRAWL_THRESHOLD.saturating_sub(consecutive_crawls);
  let crawls_above_threshold = CONSECUTIVE_CRAWL_THRESHOLD
    .saturating_sub(remaining_crawls_under_threshold);
  value
    .saturating_add(remaining_crawls_under_threshold)
    .saturating_add(crawls_above_threshold.saturating_mul(1 + PENALTY))
}

impl GroupMinMovesExpensiveCrawlsBoard {
  pub fn get(&self, square: Square) -> HeuristicValue {
    self.squares[square.0 as usize]
  }

  pub fn from_move_board(board: &MoveBoard, goal: Square) -> Self {
    let mut group_min_moves_board = [HeuristicValue::MAX; 256];
    let mut consecutive_crawls_board = [HeuristicValue::MAX; 256];
    let mut queue = VecDeque::<(Square, HeuristicValue, HeuristicValue)>::new();
    queue.push_back((Square(goal.0), 0, 0));

    while let Some((square, value, consecutive_crawls)) = queue.pop_front() {
      let square_index = square.0 as usize;

      let lowest_value = group_min_moves_board[square_index];
      let lowest_consecutive_crawls = consecutive_crawls_board[square_index];

      if lowest_value <= value
        && calc_value_after_threshold_moves(
          lowest_value,
          lowest_consecutive_crawls,
        ) <= calc_value_after_threshold_moves(value, consecutive_crawls)
      {
        continue;
      }

      // TODO: We currently keep just the combination with the lowest value.
      // We could iterate fewer times if we keep track of the lowest crawls for
      // each value that we've seen instead of just the lowest value we've seen.
      if value < lowest_value {
        group_min_moves_board[square_index] = value;
        consecutive_crawls_board[square_index] = consecutive_crawls;
      } else if value == lowest_value
        && consecutive_crawls < lowest_consecutive_crawls
      {
        consecutive_crawls_board[square_index] = consecutive_crawls;
      }

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
            value
              + if consecutive_crawls > CONSECUTIVE_CRAWL_THRESHOLD {
                1 + PENALTY
              } else {
                1
              },
            consecutive_crawls + 1,
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
  fn test_calc_value_after_threshold_moves() {
    assert_eq!(calc_value_after_threshold_moves(4, 0), 7);
    assert_eq!(calc_value_after_threshold_moves(4, 1), 7 + PENALTY);
    assert_eq!(
      calc_value_after_threshold_moves(4, 2),
      7 + PENALTY + PENALTY
    );
    assert_eq!(
      calc_value_after_threshold_moves(4, 3),
      7 + PENALTY + PENALTY + PENALTY
    );
    assert_eq!(
      calc_value_after_threshold_moves(4, 4),
      7 + PENALTY + PENALTY + PENALTY
    );
  }

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
