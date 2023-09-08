use core::fmt;

use std::collections::BTreeSet;
use std::collections::VecDeque;

use crate::mechanics::get_move_destination;
use crate::mechanics::get_movement_ray;
use crate::mechanics::ActorSquares;
use crate::mechanics::BitBoard;
use crate::mechanics::BlockBoard;
use crate::mechanics::Direction;
use crate::mechanics::Square;
use crate::solvers::SolutionStep;

struct HeuristicBoard {
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

  fn can_prune(
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

  fn from_board(board: &BlockBoard) -> Self {
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
          get_movement_ray(board, Square(source), BitBoard::ZERO, direction);
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

pub fn deepening_search_to_depth(
  board: &BlockBoard,
  actor_squares: ActorSquares,
  max_depth: usize,
) -> Option<Vec<SolutionStep>> {
  let heuristic_board = HeuristicBoard::from_board(board);

  let mut current_depth = 0;

  let mut visited = vec![0; u32::MAX as usize];

  loop {
    let depth_search_result = search_at_depth(
      board,
      &heuristic_board,
      actor_squares,
      current_depth,
      &mut visited,
    );
    if current_depth == max_depth || depth_search_result.is_some() {
      return depth_search_result;
    }
    // println!("No solution found for depth {}", current_depth);
    current_depth += 1;
  }
}

fn search_at_depth(
  board: &BlockBoard,
  heuristic_board: &HeuristicBoard,
  actor_squares: ActorSquares,
  search_depth: usize,
  visited: &mut Vec<u8>,
) -> Option<Vec<SolutionStep>> {
  let mut solution = Vec::with_capacity(search_depth);

  _search_at_depth(
    board,
    heuristic_board,
    actor_squares,
    search_depth,
    &mut solution,
    visited,
  )
}

fn _search_at_depth(
  board: &BlockBoard,
  heuristic_board: &HeuristicBoard,
  actor_squares: ActorSquares,
  search_depth: usize,
  solution: &mut Vec<SolutionStep>,
  visited: &mut Vec<u8>,
) -> Option<Vec<SolutionStep>> {
  let depth = solution.len();
  if depth == search_depth {
    return if actor_squares.0.contains(&board.goal) {
      Some(solution.clone())
    } else {
      None
    };
  }

  let depth_allowance = search_depth - depth;

  if heuristic_board.can_prune(actor_squares, depth_allowance) {
    return None;
  }

  let visited_key = actor_squares.as_sorted_u32() as usize;
  if visited[visited_key] >= depth_allowance as u8 {
    return None;
  }
  visited[visited_key] = depth_allowance as u8;

  let occupied_squares = actor_squares.as_bitboard();
  let actor_squares = actor_squares.0;

  for actor_index in 0..actor_squares.len() {
    let actor_square = actor_squares[actor_index];
    for direction in Direction::VARIANTS {
      let move_destination =
        get_move_destination(board, actor_square, occupied_squares, direction);
      if move_destination == actor_square {
        continue;
      }
      let mut new_actor_squares = actor_squares;
      new_actor_squares[actor_index] = move_destination;

      solution.push(SolutionStep {
        actor: actor_index,
        direction,
      });
      let result = _search_at_depth(
        board,
        heuristic_board,
        ActorSquares(new_actor_squares),
        search_depth,
        solution,
        visited,
      );
      if result.is_some() {
        return result;
      }
      solution.pop();
    }
  }

  None
}

// Needed to prevent long benchmarks from running during `cargo test`
#[cfg(all(feature = "benchmarks", test))]
mod benchmarks {
  extern crate test;
  use test::Bencher;

  use std::time::Instant;

  use crate::board_generators::EmptyMiddleGoalBoardGenerator;
  use crate::mechanics::WalledBoard;
  use crate::mechanics::WalledBoardPosition;
  use crate::mechanics::WalledBoardPositionGenerator;

  use super::*;

  #[bench]
  fn bench_init_already_solved(b: &mut Bencher) {
    b.iter(|| {
      deepening_search_to_depth(
        &BlockBoard::EMPTY,
        ActorSquares([Square(0), Square(1), Square(2), Square(3)]),
        1,
      )
    })
  }

  #[bench]
  fn bench_solve_generated_15(_b: &mut Bencher) {
    let walled_board = WalledBoard {
      goal: Square(184),
      vertical: [
        [
          false, false, false, false, false, false, true, false, false, false,
          false, false, true, false, false,
        ],
        [
          false, false, false, true, false, false, false, false, true, false,
          false, false, false, false, false,
        ],
        [
          false, false, false, false, false, false, false, false, false, false,
          false, false, false, false, false,
        ],
        [
          false, false, false, false, false, false, false, false, false, false,
          false, false, false, false, false,
        ],
        [
          false, false, false, false, true, false, true, false, false, false,
          false, false, false, false, false,
        ],
        [
          false, false, false, false, false, false, false, false, false, false,
          false, false, false, false, false,
        ],
        [
          false, false, false, false, false, false, false, false, false, false,
          false, false, true, true, false,
        ],
        [
          false, false, false, false, false, false, true, false, true, false,
          false, false, false, false, false,
        ],
        [
          false, false, false, false, false, false, true, false, true, false,
          false, false, false, false, false,
        ],
        [
          false, true, false, false, false, false, false, false, false, false,
          false, false, false, false, true,
        ],
        [
          false, false, false, false, false, false, true, false, false, false,
          true, false, false, false, false,
        ],
        [
          false, false, false, false, false, false, false, false, false, false,
          false, false, false, false, false,
        ],
        [
          false, false, false, false, false, false, true, false, false, false,
          false, false, false, false, false,
        ],
        [
          false, false, false, false, false, false, false, false, false, false,
          false, false, false, false, false,
        ],
        [
          false, false, false, false, false, false, false, false, false, false,
          false, false, false, true, false,
        ],
        [
          false, false, true, false, false, false, false, false, false, false,
          false, false, true, false, false,
        ],
      ],
      horizontal: [
        [
          false, true, false, false, false, false, false, false, true, false,
          false, false, false, false, false,
        ],
        [
          false, false, false, false, false, false, false, false, false, true,
          false, false, false, false, false,
        ],
        [
          false, false, false, false, false, false, false, false, false, false,
          false, false, false, false, false,
        ],
        [
          false, false, false, false, false, false, false, false, false, false,
          false, false, false, false, false,
        ],
        [
          true, false, false, false, false, false, false, false, false, false,
          false, false, false, false, false,
        ],
        [
          false, false, false, false, true, false, false, false, false, false,
          false, false, false, false, false,
        ],
        [
          false, false, false, true, false, false, false, false, false, false,
          true, true, false, false, false,
        ],
        [
          false, false, false, false, false, false, true, false, true, false,
          false, false, false, false, false,
        ],
        [
          false, true, false, false, false, false, true, false, true, false,
          false, false, false, false, false,
        ],
        [
          false, false, false, false, false, false, false, false, false, false,
          false, false, false, false, false,
        ],
        [
          false, false, false, false, false, false, false, false, false, true,
          false, false, false, false, false,
        ],
        [
          false, false, false, false, false, false, false, false, false, false,
          false, false, false, false, false,
        ],
        [
          false, false, false, false, false, true, false, false, false, false,
          false, false, false, false, false,
        ],
        [
          false, false, false, false, false, false, false, false, false, false,
          false, false, false, true, false,
        ],
        [
          false, false, false, false, false, false, true, false, true, false,
          false, false, false, false, false,
        ],
        [
          false, false, false, false, true, false, false, false, false, false,
          true, false, false, false, false,
        ],
      ],
    };
    let actor_squares = ActorSquares([37, 108, 57, 50].map(Square));

    let board = BlockBoard::from(walled_board);

    let start = Instant::now();

    let solution: Option<Vec<SolutionStep>> =
      deepening_search_to_depth(&board, actor_squares, 45);
    assert_eq!(solution.map(|v| v.len()), Some(15));

    let elapsed = start.elapsed();
    println!("#######");
    println!("Elapsed: {:.2?}", elapsed);
  }

  #[bench]
  fn bench_solve_empty_middle_goal(_b: &mut Bencher) {
    let position = EmptyMiddleGoalBoardGenerator::new().generate_position();
    let WalledBoardPosition {
      walled_board,
      actor_squares,
    } = position;
    let board = BlockBoard::from(walled_board);

    let start = Instant::now();

    let solution: Option<Vec<SolutionStep>> =
      deepening_search_to_depth(&board, actor_squares, 45);
    assert_eq!(solution.map(|v| v.len()), Some(41));

    let elapsed = start.elapsed();
    println!("#######");
    println!("Elapsed: {:.2?}", elapsed);
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_already_solved() {
    let solution = deepening_search_to_depth(
      &BlockBoard::EMPTY,
      ActorSquares([Square(0), Square(1), Square(2), Square(3)]),
      1,
    );
    assert_eq!(solution, Some(vec![]));
  }

  #[test]
  fn test_empty_solve_in_one() {
    let solution = deepening_search_to_depth(
      &BlockBoard::EMPTY,
      ActorSquares([Square(1), Square(2), Square(3), Square(4)]),
      1,
    );
    assert_eq!(solution, Some(vec![(0, Direction::Left).into()]));
  }

  #[test]
  fn test_empty_solve_in_one_reverse() {
    let solution = deepening_search_to_depth(
      &BlockBoard::EMPTY,
      ActorSquares([Square(4), Square(3), Square(2), Square(1)]),
      1,
    );
    assert_eq!(solution, Some(vec![(3, Direction::Left).into()]));
  }

  #[test]
  fn test_empty_solve_in_one_below_max() {
    let solution = deepening_search_to_depth(
      &BlockBoard::EMPTY,
      ActorSquares([Square(4), Square(3), Square(2), Square(1)]),
      10,
    );
    assert_eq!(solution, Some(vec![(3, Direction::Left).into()]));
  }

  #[test]
  fn test_empty_no_solve_in_one() {
    let solution = deepening_search_to_depth(
      &BlockBoard::EMPTY,
      ActorSquares([Square(17), Square(18), Square(19), Square(20)]),
      1,
    );
    assert_eq!(solution, None);
  }

  #[test]
  fn test_empty_solve_in_two() {
    let solution = deepening_search_to_depth(
      &BlockBoard::EMPTY,
      ActorSquares([Square(17), Square(18), Square(19), Square(20)]),
      2,
    );
    assert_eq!(
      solution,
      Some(vec![(0, Direction::Up).into(), (0, Direction::Left).into()])
    );
  }

  #[test]
  fn test_empty_solve_in_three() {
    let board = BlockBoard {
      goal: Square(17),
      ..BlockBoard::EMPTY
    };
    let solution = deepening_search_to_depth(
      &board,
      ActorSquares([Square(14), Square(15), Square(49), Square(255)]),
      3,
    );
    assert_eq!(
      solution,
      Some(vec![
        (0, Direction::Left).into(),
        (1, Direction::Left).into(),
        (2, Direction::Up).into()
      ])
    );
  }
}
