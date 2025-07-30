use core::fmt;

use crate::mechanics::ActorSquares;
use crate::mechanics::Direction;
use crate::mechanics::MoveBoard;
use crate::mechanics::Square;
use crate::solvers::format_square_heuristics;

use super::get_min;
use super::Heuristic;
use super::HeuristicValue;

pub struct GroupMinMovesBoard {
  pub squares: [HeuristicValue; 256],
}

impl fmt::Debug for GroupMinMovesBoard {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    format_square_heuristics(f, &self.squares)
  }
}

impl GroupMinMovesBoard {
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
          let move_destination =
            board.get_unimpeded_move_destination(square, direction);
          if move_destination == square {
            next_square_set.extend(board.get_unimpeded_movement_ray_squares(
              square,
              direction.opposite(),
            ));
          } else {
            next_square_set.push(
              square
                .get_adjacent(direction)
                .expect("Adjacent must exist if movement is unimpeded"),
            )
          }
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

impl Heuristic for GroupMinMovesBoard {
  // This is an admissible heuristic because GroupMinMovesBoard is constructed
  // such that if the heuristic of a given actor arrangement is N, then it is
  // impossible to make a move where the heuristic of the new arrangement is
  // less than N-1. In other words, an arrangement yielding heuristic M is
  // unreachable without first creating an arrangement yielding heuristic M+1.
  // This also means no single move can decrease our heuristic by more than one.
  // If having an actor on the goal yields heuristic zero, the above means our
  // heuristic is always admissible.
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
      let board = GroupMinMovesBoard::from_move_board(
        &MoveBoard::from(&position.walled_board),
        position.goal,
      );
      insta::assert_debug_snapshot!(name, board);
    }
  }
}
