use core::fmt;
use std::cmp::min;

use crate::mechanics::ActorSquares;
use crate::mechanics::Direction;
use crate::mechanics::MoveBoard;
use crate::mechanics::Square;

pub struct CrawlAwareImprovedHeuristicBoard {
  pub heuristics: [usize; 256],
  pub crawls: [usize; 256],
}

impl fmt::Debug for CrawlAwareImprovedHeuristicBoard {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let strings = self.heuristics.map(|x| format!("{:03}", x));
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

impl CrawlAwareImprovedHeuristicBoard {
  pub fn get_heuristic(&self, actor_squares: ActorSquares) -> usize {
    actor_squares
      .0
      .iter()
      .map(|square| self.heuristics[square.0 as usize])
      .min()
      .unwrap()
  }

  pub fn from_move_board(board: &MoveBoard, goal: Square) -> Self {
    let mut square_crawls = [255; 256];
    let mut square_heuristics = [255; 256];
    let mut current_iteration = 0;
    let mut current_square_set = vec![(Square(goal.0), 0)];
    let mut next_square_set = Vec::new();

    while !current_square_set.is_empty() {
      for &(square, crawls) in current_square_set.iter() {
        let square_index = square.0 as usize;

        if square_heuristics[square_index] <= current_iteration
          && square_crawls[square_index] <= crawls
        {
          continue;
        }
        square_heuristics[square_index] =
          min(square_heuristics[square_index], current_iteration);
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
      current_iteration += 1;
      current_square_set = next_square_set;
      next_square_set = Vec::new();
    }

    Self {
      heuristics: square_heuristics,
      crawls: square_crawls,
    }
  }
}
