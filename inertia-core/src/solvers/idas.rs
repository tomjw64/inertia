use crate::mechanics::ActorSquares;
use crate::mechanics::Direction;
use crate::mechanics::MoveBoard;
use crate::mechanics::Square;
use crate::solvers::SolutionStep;

use crate::solvers::HeuristicBoard;

pub fn deepening_search_to_depth(
  board: &MoveBoard,
  goal: Square,
  actor_squares: ActorSquares,
  max_depth: usize,
) -> Option<Vec<SolutionStep>> {
  let heuristic_board = HeuristicBoard::from_move_board(board, goal);

  let mut current_depth = 0;

  let mut visited = vec![0; u32::MAX as usize];

  loop {
    let mut solution = Vec::with_capacity(current_depth);
    let depth_search_result = search_at_depth(
      board,
      goal,
      &heuristic_board,
      actor_squares,
      current_depth,
      &mut solution,
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
  board: &MoveBoard,
  goal: Square,
  heuristic_board: &HeuristicBoard,
  actor_squares: ActorSquares,
  search_depth: usize,
  solution: &mut Vec<SolutionStep>,
  visited: &mut Vec<u8>,
) -> Option<Vec<SolutionStep>> {
  let depth = solution.len();
  if depth == search_depth {
    return if actor_squares.0.contains(&goal) {
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

  for actor_index in 0..actor_squares.0.len() {
    let actor_square = actor_squares.0[actor_index];
    for direction in Direction::VARIANTS {
      let move_destination =
        board.get_move_destination(actor_square, actor_squares, direction);
      if move_destination == actor_square {
        continue;
      }
      let mut new_actor_squares = actor_squares;
      new_actor_squares.0[actor_index] = move_destination;

      solution.push(SolutionStep {
        actor: actor_index as u8,
        direction,
      });
      let result = search_at_depth(
        board,
        goal,
        heuristic_board,
        new_actor_squares,
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
  use crate::mechanics::Position;
  use crate::mechanics::PositionGenerator;
  use crate::mechanics::WalledBoard;

  use super::*;

  #[bench]
  fn bench_solve_generated_15(_b: &mut Bencher) {
    println!("#######");
    let walled_board = WalledBoard {
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

    let board = MoveBoard::from(&walled_board);

    let start = Instant::now();

    let solution: Option<Vec<SolutionStep>> =
      deepening_search_to_depth(&board, Square(184), actor_squares, 45);
    assert_eq!(solution.map(|v| v.len()), Some(15));

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
  }

  #[bench]
  fn bench_solve_shuffle_puzzle(_b: &mut Bencher) {
    println!("#######");
    let mut vertical = [[false; 15]; 16];
    vertical[0] = [true; 15];
    vertical[15] = [true; 15];
    let mut horizontal = [[false; 15]; 16];
    horizontal[0] = [true; 15];
    horizontal[15] = [true; 15];
    let walled_board = WalledBoard {
      vertical,
      horizontal,
    };
    let actor_squares =
      ActorSquares([Square(17), Square(18), Square(33), Square(34)]);
    let goal = Square::from_row_col(8, 8);

    let board = MoveBoard::from(&walled_board);

    let start = Instant::now();

    let solution: Option<Vec<SolutionStep>> =
      deepening_search_to_depth(&board, goal, actor_squares, 80);
    assert_eq!(solution.map(|v| v.len()), Some(70));

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
  }

  #[bench]
  fn bench_solve_empty_middle_goal(_b: &mut Bencher) {
    println!("#######");
    let position = EmptyMiddleGoalBoardGenerator::new().generate_position();
    let Position {
      walled_board,
      actor_squares,
      goal,
    } = position;
    let board = MoveBoard::from(&walled_board);

    let start = Instant::now();

    let solution: Option<Vec<SolutionStep>> =
      deepening_search_to_depth(&board, goal, actor_squares, 45);
    assert_eq!(solution.map(|v| v.len()), Some(41));

    let elapsed = start.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_already_solved() {
    let solution = deepening_search_to_depth(
      &MoveBoard::EMPTY,
      Square(0),
      ActorSquares([Square(0), Square(1), Square(2), Square(3)]),
      1,
    );
    assert_eq!(solution, Some(vec![]));
  }

  #[test]
  fn test_empty_solve_in_one() {
    let solution = deepening_search_to_depth(
      &MoveBoard::EMPTY,
      Square(0),
      ActorSquares([Square(1), Square(2), Square(3), Square(4)]),
      1,
    );
    assert_eq!(solution, Some(vec![(0, Direction::Left).into()]));
  }

  #[test]
  fn test_empty_solve_in_one_reverse() {
    let solution = deepening_search_to_depth(
      &MoveBoard::EMPTY,
      Square(0),
      ActorSquares([Square(4), Square(3), Square(2), Square(1)]),
      1,
    );
    assert_eq!(solution, Some(vec![(3, Direction::Left).into()]));
  }

  #[test]
  fn test_empty_solve_in_one_below_max() {
    let solution = deepening_search_to_depth(
      &MoveBoard::EMPTY,
      Square(0),
      ActorSquares([Square(4), Square(3), Square(2), Square(1)]),
      10,
    );
    assert_eq!(solution, Some(vec![(3, Direction::Left).into()]));
  }

  #[test]
  fn test_empty_no_solve_in_one() {
    let solution = deepening_search_to_depth(
      &MoveBoard::EMPTY,
      Square(0),
      ActorSquares([Square(17), Square(18), Square(19), Square(20)]),
      1,
    );
    assert_eq!(solution, None);
  }

  #[test]
  fn test_empty_solve_in_two() {
    let solution = deepening_search_to_depth(
      &MoveBoard::EMPTY,
      Square(0),
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
    let solution = deepening_search_to_depth(
      &MoveBoard::EMPTY,
      Square(17),
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
