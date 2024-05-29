use crate::mechanics::ActorSquares;
use crate::mechanics::Direction;
use crate::mechanics::MoveBoard;
use crate::mechanics::Square;
use crate::solvers::HeuristicBoard;
use crate::solvers::SolutionStep;

pub fn deepening_search_to_depth(
  board: &MoveBoard,
  goal: Square,
  actor_squares: ActorSquares,
  max_depth: usize,
) -> Option<Vec<SolutionStep>> {
  let heuristic_board = HeuristicBoard::from_move_board(board, goal);

  let mut current_search_depth_limit = 0;

  let mut visited = vec![0; u32::MAX as usize];

  while current_search_depth_limit <= max_depth {
    let mut solution: Vec<SolutionStep> =
      Vec::with_capacity(current_search_depth_limit);
    let mut stack: Vec<(usize, ActorSquares, SolutionStep)> =
      Vec::with_capacity(
        current_search_depth_limit * (actor_squares.0.len() - 1) + 1,
      );

    if actor_squares.0.contains(&goal) {
      return Some(solution);
    }

    let visited_key = actor_squares.as_sorted_u32() as usize;
    visited[visited_key] = current_search_depth_limit as u8;

    for actor in 0..actor_squares.0.len() {
      for direction in Direction::VARIANTS {
        stack.push((
          0,
          actor_squares,
          SolutionStep {
            actor: actor as u8,
            direction,
          },
        ))
      }
    }

    while let Some((at_depth, actor_squares, step_to_make)) = stack.pop() {
      solution.truncate(at_depth);
      solution.push(step_to_make);

      let SolutionStep { actor, direction } = step_to_make;
      let actor = actor as usize;
      let actor_square = actor_squares.0[actor];

      let move_destination =
        board.get_move_destination(actor_square, actor_squares, direction);
      if move_destination == actor_square {
        continue;
      }

      if solution.len() == current_search_depth_limit {
        if move_destination == goal {
          return Some(solution.clone());
        } else {
          continue;
        };
      }

      let mut next_actor_squares = actor_squares;
      next_actor_squares.0[actor] = move_destination;

      let depth_allowance = current_search_depth_limit - at_depth;

      if heuristic_board.can_prune(next_actor_squares, depth_allowance) {
        continue;
      }

      let visited_key = next_actor_squares.as_sorted_u32() as usize;
      if visited[visited_key] >= depth_allowance as u8 {
        continue;
      }
      visited[visited_key] = depth_allowance as u8;

      for next_actor in 0..next_actor_squares.0.len() {
        for next_direction in Direction::VARIANTS {
          if next_actor == actor
            && (next_direction == direction
              || next_direction == direction.opposite())
          {
            continue;
          }
          stack.push((
            at_depth + 1,
            next_actor_squares,
            SolutionStep {
              actor: next_actor as u8,
              direction: next_direction,
            },
          ))
        }
      }
    }
    // println!("No solution found for depth {}", current_search_depth_limit);
    current_search_depth_limit += 1
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
  use crate::solvers::fixtures::GENERATED_WALLED_BOARD_15;

  use super::*;

  #[bench]
  fn bench_solve_generated_15(_b: &mut Bencher) {
    let walled_board = GENERATED_WALLED_BOARD_15;
    let actor_squares = ActorSquares([37, 108, 57, 50].map(Square));
    let board = MoveBoard::from(&walled_board);

    let start = Instant::now();

    let solution: Option<Vec<SolutionStep>> =
      deepening_search_to_depth(&board, Square(184), actor_squares, 45);
    assert_eq!(solution.map(|v| v.len()), Some(15));

    let elapsed = start.elapsed();
    println!("#######");
    println!("Elapsed: {:.2?}", elapsed);
  }

  #[bench]
  fn bench_solve_empty_middle_goal(_b: &mut Bencher) {
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
    assert_eq!(solution.map(|v| v.len()), Some(1));
  }

  #[test]
  fn test_empty_solve_in_one_reverse() {
    let solution = deepening_search_to_depth(
      &MoveBoard::EMPTY,
      Square(0),
      ActorSquares([Square(4), Square(3), Square(2), Square(1)]),
      1,
    );
    assert_eq!(solution.map(|v| v.len()), Some(1));
  }

  #[test]
  fn test_empty_solve_in_one_below_max() {
    let solution = deepening_search_to_depth(
      &MoveBoard::EMPTY,
      Square(0),
      ActorSquares([Square(4), Square(3), Square(2), Square(1)]),
      10,
    );
    assert_eq!(solution.map(|v| v.len()), Some(1));
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
    assert_eq!(solution.map(|v| v.len()), Some(2));
  }

  #[test]
  fn test_empty_solve_in_three() {
    let solution = deepening_search_to_depth(
      &MoveBoard::EMPTY,
      Square(17),
      ActorSquares([Square(14), Square(15), Square(49), Square(255)]),
      3,
    );
    assert_eq!(solution.map(|v| v.len()), Some(3));
  }
}
