use core::fmt;
use std::collections::BTreeSet;
use std::collections::VecDeque;

use crate::mechanics::ActorSquares;
use crate::mechanics::BitBoard;
use crate::mechanics::BlockBoard;
use crate::mechanics::Direction;
use crate::mechanics::Square;

pub struct HeuristicBoard {
  squares: [usize; 256],
  heuristic_augment_unoccupied_bitboards: Vec<(usize, BitBoard)>,
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
    let base_heurisitc = self.get(actor_squares);
    // Since the heuristic is admissable (and cost per move is constant), if the
    // heuristic is greater than the cost/depth allowance remaining for this
    // search depth, we know with certainty we will not reach the goal in this
    // iteration and so we can prune this branch.
    if base_heurisitc > depth_allowance {
      return true;
    }

    // Since our heuristic, at least for the moment, assumes perfectly placed
    // actors, we can quickly bump that up by 1 if a known location for a
    // perfectly placed actor is not occupied, that is, any square next to the
    // goal, if the goal has no wall next to it. We can expand this idea to 2 by
    // considering the squares adjacent to the goal. To 3 by considering the
    // squares adjecent to those squares and so on and so forth.
    let actor_squares_bitboard = actor_squares.as_bitboard();
    for (augment_amount, unoccupied_augment_bitboard) in
      self.heuristic_augment_unoccupied_bitboards.iter()
    {
      if base_heurisitc + augment_amount > depth_allowance
        && (actor_squares_bitboard & unoccupied_augment_bitboard).is_zero()
      {
        return true;
      }
    }

    false
  }
}

impl From<&BlockBoard> for HeuristicBoard {
  fn from(board: &BlockBoard) -> Self {
    let goal_index = board.goal.0;

    let mut unoccupied_augment_bitboards: Vec<(usize, BitBoard)> = vec![];
    let mut current_augment_amount = 1;
    let mut current_square_set = BTreeSet::new();
    current_square_set.insert(board.goal);

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
      unoccupied_augment_bitboards
        .push((current_augment_amount, unoccupied_augment_bitboard));
      current_augment_amount += 1;
    }
    unoccupied_augment_bitboards.reverse();

    let mut heuristic_board = Self {
      squares: [255; 256],
      heuristic_augment_unoccupied_bitboards: unoccupied_augment_bitboards,
    };

    heuristic_board.squares[goal_index as usize] = 0;

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
          board.get_movement_ray(Square(source), BitBoard::ZERO, direction);
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
