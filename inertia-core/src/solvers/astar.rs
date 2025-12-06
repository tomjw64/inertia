use std::borrow::Borrow;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::mechanics::ActorSquares;
use crate::mechanics::Direction;
use crate::mechanics::MoveBoard;
use crate::mechanics::Position;
use crate::mechanics::Square;
use crate::solvers::roll_zobrist_hash;
use crate::solvers::zobrist_hash;
use crate::solvers::BucketingPriorityQueue;
use crate::solvers::GroupMinMovesExpensiveCrawlsBoard;
use crate::solvers::Heuristic;
use crate::solvers::NoopHasherBuilder;
use crate::solvers::Solution;
use crate::solvers::SolutionStep;

struct VisitedData {
  parent: ActorSquares,
  depth: u8,
}

struct QueueData {
  actor_squares: ActorSquares,
  depth: u8,
}

pub fn solve_position<P: Borrow<Position>>(
  position: P,
  max_depth: usize,
) -> Option<Solution> {
  let Position {
    walled_board,
    actor_squares,
    goal,
  } = position.borrow();
  let board = MoveBoard::from(walled_board);
  solve(&board, *goal, *actor_squares, max_depth)
}

pub fn solve(
  board: &MoveBoard,
  goal: Square,
  actor_squares: ActorSquares,
  max_depth: usize,
) -> Option<Solution> {
  let heuristic_board =
    GroupMinMovesExpensiveCrawlsBoard::from_move_board(board, goal);
  let mut queue = BucketingPriorityQueue::with_capacities(256, 1024);
  let mut visited: HashMap<u64, VisitedData, NoopHasherBuilder> =
    HashMap::with_capacity_and_hasher(1024, NoopHasherBuilder::default());

  queue.push(
    QueueData {
      actor_squares,
      depth: 0,
    },
    heuristic_board.get_heuristic(actor_squares) as usize,
  );

  let mut visited_nodes = 0;
  while let Some(queue_data) = queue.pop() {
    visited_nodes += 1;
    let QueueData {
      actor_squares,
      depth,
    } = queue_data;

    if depth as usize > max_depth {
      return None;
    }

    if actor_squares.0.contains(&goal) {
      print!("[{} visited nodes] ", visited_nodes);
      return Some(reconstruct_solution(actor_squares, depth, &visited));
    }

    let depth_after_move = depth + 1;
    let parent_hash = zobrist_hash(actor_squares.as_bytes());
    let move_destinations =
      board.get_all_actor_move_destinations(actor_squares);
    for actor_index in 0..4 {
      let actor_square = actor_squares.0[actor_index];
      let move_destinations = move_destinations[actor_index];
      for move_index in 0..4 {
        let move_destination = move_destinations[move_index];
        if move_destination == actor_square {
          continue;
        }
        let mut new_actor_squares = actor_squares;
        new_actor_squares.0[actor_index] = move_destination;

        let visited_key =
          roll_zobrist_hash(parent_hash, actor_square.0, move_destination.0);
        let visited_entry = visited.entry(visited_key);
        let skippable = match visited_entry {
          Entry::Occupied(mut entry) => {
            let existing: &mut VisitedData = entry.get_mut();
            if existing.depth <= depth_after_move {
              true
            } else {
              existing.depth = depth_after_move;
              existing.parent = actor_squares;
              false
            }
          }
          Entry::Vacant(entry) => {
            entry.insert(VisitedData {
              depth: depth_after_move,
              parent: actor_squares,
            });
            false
          }
        };
        if skippable {
          continue;
        }

        queue.push(
          QueueData {
            actor_squares: new_actor_squares,
            depth: depth_after_move,
          },
          depth_after_move as usize
            + heuristic_board.get_heuristic(new_actor_squares) as usize,
        );
      }
    }
  }

  print!("[{} visited nodes] ", visited_nodes);
  None
}

fn reconstruct_solution(
  actor_squares: ActorSquares,
  depth: u8,
  visited: &HashMap<u64, VisitedData, NoopHasherBuilder>,
) -> Solution {
  let mut solution_steps = Vec::with_capacity(depth as usize);
  let mut current_actor_squares = actor_squares;
  for _ in 0..depth {
    let visited_key = zobrist_hash(current_actor_squares.as_bytes());
    let parent = visited
      .get(&visited_key)
      .expect("parent must be visited")
      .parent;
    for idx in 0..4 {
      if current_actor_squares.0[idx] != parent.0[idx] {
        solution_steps.push(SolutionStep {
          actor: idx as u8,
          direction: match current_actor_squares.0[idx].0 as i16
            - parent.0[idx].0 as i16
          {
            -255..=-16 => Direction::Up,
            -15..=-1 => Direction::Left,
            1..=15 => Direction::Right,
            16..=255 => Direction::Down,
            _ => unreachable!(),
          },
        })
      }
    }
    current_actor_squares = parent;
  }
  solution_steps.reverse();
  Solution(solution_steps)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_already_solved() {
    let solution = solve(
      &MoveBoard::EMPTY,
      Square(0),
      ActorSquares([Square(0), Square(1), Square(2), Square(3)]),
      1,
    );
    assert_eq!(solution, Some(Solution(vec![])));
  }

  #[test]
  fn test_empty_solve_in_one() {
    let solution = solve(
      &MoveBoard::EMPTY,
      Square(0),
      ActorSquares([Square(1), Square(2), Square(3), Square(4)]),
      1,
    );
    assert_eq!(solution, Some(Solution(vec![(0, Direction::Left).into()])));
  }

  #[test]
  fn test_empty_solve_in_one_reverse() {
    let solution = solve(
      &MoveBoard::EMPTY,
      Square(0),
      ActorSquares([Square(4), Square(3), Square(2), Square(1)]),
      1,
    );
    assert_eq!(solution, Some(Solution(vec![(3, Direction::Left).into()])));
  }

  #[test]
  fn test_empty_solve_in_one_below_max() {
    let solution = solve(
      &MoveBoard::EMPTY,
      Square(0),
      ActorSquares([Square(4), Square(3), Square(2), Square(1)]),
      10,
    );
    assert_eq!(solution, Some(Solution(vec![(3, Direction::Left).into()])));
  }

  #[test]
  fn test_empty_no_solve_in_one() {
    let solution = solve(
      &MoveBoard::EMPTY,
      Square(0),
      ActorSquares([Square(17), Square(18), Square(19), Square(20)]),
      1,
    );
    assert_eq!(solution, None);
  }

  #[test]
  fn test_empty_solve_in_two() {
    let solution = solve(
      &MoveBoard::EMPTY,
      Square(0),
      ActorSquares([Square(17), Square(18), Square(19), Square(20)]),
      2,
    );
    assert_eq!(
      solution,
      Some(Solution(vec![
        (3, Direction::Up).into(),
        (3, Direction::Left).into()
      ]))
    );
  }

  #[test]
  fn test_empty_solve_in_three() {
    let solution = solve(
      &MoveBoard::EMPTY,
      Square(17),
      ActorSquares([Square(14), Square(15), Square(49), Square(255)]),
      3,
    );
    assert_eq!(
      solution,
      Some(Solution(vec![
        (0, Direction::Left).into(),
        (1, Direction::Left).into(),
        (2, Direction::Up).into()
      ]))
    );
  }
}
