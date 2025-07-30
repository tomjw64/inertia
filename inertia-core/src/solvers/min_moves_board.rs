use core::fmt;

use crate::mechanics::ActorSquares;
use crate::mechanics::Direction;
use crate::mechanics::MoveBoard;
use crate::mechanics::Square;
use crate::solvers::format_square_heuristics;

use super::get_min;
use super::Heuristic;
use super::HeuristicValue;

pub struct MinMovesBoard {
  pub squares: [HeuristicValue; 256],
}

impl fmt::Debug for MinMovesBoard {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    format_square_heuristics(f, &self.squares)
  }
}

impl MinMovesBoard {
  pub fn get(&self, square: Square) -> HeuristicValue {
    self.squares[square.0 as usize]
  }

  pub fn from_move_board(board: &MoveBoard, goal: Square) -> Self {
    let mut square_min_moves = [HeuristicValue::MAX; 256];
    let mut current_iteration = 0;
    let mut current_square_set = vec![Square(goal.0)];
    let mut next_square_set = Vec::new();

    while !current_square_set.is_empty() {
      for &square in current_square_set.iter() {
        let square_index = square.0 as usize;

        if square_min_moves[square_index] <= current_iteration {
          continue;
        }
        square_min_moves[square_index] = current_iteration;

        for direction in Direction::VARIANTS {
          next_square_set.extend(
            board.get_unimpeded_movement_ray_squares(square, direction),
          );
        }
      }
      current_iteration += 1;
      current_square_set = next_square_set;
      next_square_set = Vec::new();
    }

    Self {
      squares: square_min_moves,
    }
  }
}

impl Heuristic for MinMovesBoard {
  // This is an admissible heuristic because the value for each square
  // represents the number of moves it would take for an actor on that square to
  // move to the goal square if it could stop anywhere, i.e. if we had an
  // unlimited number of assisting actors that were already perfectly placed.
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
      let board = MinMovesBoard::from_move_board(
        &MoveBoard::from(&position.walled_board),
        position.goal,
      );
      insta::assert_debug_snapshot!(name, board);
    }
  }
}
