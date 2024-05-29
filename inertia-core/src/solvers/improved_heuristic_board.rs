use core::fmt;
use std::mem;

use crate::mechanics::ActorSquares;
use crate::mechanics::Direction;
use crate::mechanics::MoveBoard;
use crate::mechanics::Square;

pub struct ImprovedHeuristicBoard {
  squares: [usize; 256],
}

impl fmt::Debug for ImprovedHeuristicBoard {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let strings = self.squares.map(|x| format!("{:03}", x));
    let rows = strings.chunks(16).collect::<Vec<_>>();
    f.write_str("\n")?;
    for row in rows {
      f.write_str("[")?;
      f.write_str(&row.join(", "))?;
      f.write_str("]\n")?;
    }
    Ok(())
  }
}

impl ImprovedHeuristicBoard {
  // This is an admissible heuristic (but ONLY if any actor can reach the goal)
  // because it is constructed such that if the heuristic of a given actor
  // arrangement is N, then it is impossible to make a move where the heuristic
  // of the new arrangement is less than N-1. In other words, an arrangement
  // giving heuristic M is unreachable without first creating an arrangement
  // giving heuristic M+1. If only a single actor can reach the goal, an
  // admissible heuristic would be given by
  // max(improved_heuristic_board.get(squares), heuristic_board[i]), with i
  // being the square of the relevant actor. This is because
  // ImprovedHeuristicBoard provides an admissible heuristic for ANY actor to
  // reach the goal, but is not admissible if considering only a single actor
  // square in isolation. HeuristicBoard, however, can provide an admissible
  // heuristic for any actor on any square in isolation (but it is always
  // smaller). max(improved_heuristic_board.get(squares), heuristic_board[i]),
  // in plain terms: "The target actor cannot reach the goal in fewer moves than
  // it would take for ANY actor to reach the goal, nor can it reach the goal in
  // fewer moves than an actor at its square could if it could stop at any point."
  pub fn get(&self, actor_squares: ActorSquares) -> usize {
    actor_squares
      .0
      .iter()
      .map(|square| self.squares[square.0 as usize])
      .min()
      .unwrap()
  }

  pub fn can_prune(
    &self,
    actor_squares: ActorSquares,
    depth_allowance: usize,
  ) -> bool {
    self.get(actor_squares) > depth_allowance
  }

  pub fn from_move_board(board: &MoveBoard, goal: Square) -> Self {
    let mut visited = [false; 256];
    let mut square_heuristics = [255; 256];
    let mut current_iteration = 0;
    let mut current_square_set = vec![Square(goal.0)];
    let mut next_square_set = Vec::new();

    while current_square_set.len() > 0 {
      for &square in current_square_set.iter() {
        let square_index = square.0 as usize;
        if visited[square_index] {
          continue;
        }
        visited[square_index] = true;
        square_heuristics[square_index] = current_iteration;
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
      squares: square_heuristics,
    }
  }
}
