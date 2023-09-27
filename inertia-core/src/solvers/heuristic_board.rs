use core::fmt;
use std::collections::BTreeSet;
use std::collections::VecDeque;

use crate::mechanics::ActorSquares;
use crate::mechanics::BitBoard;
use crate::mechanics::Direction;
use crate::mechanics::MoveBoard;
use crate::mechanics::Square;

pub struct HeuristicBoard {
  squares: [usize; 256],
  heuristic_augment_unoccupied_bitboards: [Option<BitBoard>; 16],
}

impl fmt::Debug for HeuristicBoard {
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

impl HeuristicBoard {
  fn get(&self, actor_squares: ActorSquares) -> usize {
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
    let base_heuristic = self.get(actor_squares);
    // Since the heuristic is admissable (and cost per move is constant), if the
    // heuristic is greater than the cost/depth allowance remaining for this
    // search depth, we know with certainty we will not reach the goal in this
    // iteration and so we can prune this branch.
    if base_heuristic > depth_allowance {
      return true;
    }

    // Since our heuristic, at least for the moment, assumes perfectly placed
    // actors, we can quickly bump that up by 1 if a known location for a
    // perfectly placed actor is not occupied, that is, any square next to the
    // goal, if the goal has no wall next to it. We can expand this idea to 2 by
    // considering the squares adjacent to the goal. To 3 by considering the
    // squares adjecent to those squares and so on and so forth.
    // Relevant HeuristicBoard invariants for this:
    //  - a) the augment bitboards are sorted in order of ascending augment
    //    amount
    //  - b) if the bitboard with the lowest sufficient augment amount
    //    won't work then neither will any bitboard with a higher augment amount
    let heuristic_difference = depth_allowance - base_heuristic;
    if heuristic_difference >= 16 {
      return false;
    }
    if let Some(unoccupied_augment_bitboard) =
      self.heuristic_augment_unoccupied_bitboards[heuristic_difference]
    {
      return (actor_squares.as_bitboard() & unoccupied_augment_bitboard)
        .0
        .is_zero();
    }

    false
  }

  pub fn from_move_board(board: &MoveBoard, goal: Square) -> Self {
    let mut heuristic_board = Self {
      squares: [255; 256],
      heuristic_augment_unoccupied_bitboards: [None; 16],
    };

    let goal_index = goal.0;
    heuristic_board.squares[goal_index as usize] = 0;

    let mut heuristic_difference = 0;
    let mut current_square_set = BTreeSet::new();
    current_square_set.insert(goal);

    loop {
      let square_set_has_no_blocks = current_square_set
        .iter()
        .cloned()
        .all(|square| !board.has_any_block_on(square));
      let square_set_adj = current_square_set
        .iter()
        .cloned()
        .map(Square::get_adjacent_and_self)
        .collect::<Vec<_>>();
      let square_set_not_on_edge =
        square_set_adj.iter().all(|adj| adj.len() == 5);
      let can_augment = square_set_has_no_blocks && square_set_not_on_edge;
      if !can_augment {
        break;
      }
      current_square_set = current_square_set
        .iter()
        .cloned()
        .flat_map(Square::get_adjacent_and_self)
        .collect::<BTreeSet<_>>();
      let unoccupied_augment_bitboard = current_square_set
        .iter()
        .cloned()
        .map(BitBoard::from)
        .fold(BitBoard::ZERO, std::ops::BitOr::bitor);
      heuristic_board.heuristic_augment_unoccupied_bitboards
        [heuristic_difference] = Some(unoccupied_augment_bitboard);
      heuristic_difference += 1;
    }

    let mut queue: VecDeque<(usize, u8)> =
      VecDeque::from(vec![(1, goal_index)]);

    // This constructs a board which assigns an admissable heuristic to every
    // square on the board. The heuristics is admissable because it represents
    // the minimum possible number of moves it would take to reach the goal from
    // each square assuming perfect placement of all walls and other actors.
    // IDEA: This heuristic assumes perfect placement of an infinite number of
    // other actors. However, it's rare that the actors are perfectly placed, or
    // even close to being perfectly placed. Perhaps there is an efficient way
    // to increase the heuristic based on the position dynamically while keeping
    // it admissable.
    while let Some((depth, source)) = queue.pop_front() {
      let current_square_heuristic = heuristic_board.squares[source as usize];
      for direction in Direction::VARIANTS {
        let movement_ray =
          board.get_unimpeded_movement_ray(Square(source), direction);
        let mut ray_heurisitcs = [0; 256];
        ray_heurisitcs
          .iter_mut()
          .enumerate()
          .for_each(|(bit_index, value)| {
            if movement_ray.bit(bit_index) {
              *value = current_square_heuristic + 1;
            }
          });
        for index in 0u8..=255 {
          if ray_heurisitcs[index as usize] > 0
            && heuristic_board.squares[index as usize] == 255
          {
            heuristic_board.squares[index as usize] =
              ray_heurisitcs[index as usize];
            queue.push_back((depth + 1, index));
          }
        }
      }
    }
    heuristic_board
  }
}
