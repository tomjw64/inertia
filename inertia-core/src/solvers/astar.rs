use std::borrow::Borrow;

use hashbrown::hash_map::Entry;
use hashbrown::HashMap;

use crate::mechanics::ActorSquares;
use crate::mechanics::Direction;
use crate::mechanics::MoveBoard;
use crate::mechanics::Position;
use crate::mechanics::Square;
use crate::solvers::Solution;
use crate::solvers::SolutionStep;

use super::BucketingPriorityQueue;
// use super::CombinedHeuristic;
use super::GroupMinMovesBoard;
use super::Heuristic;

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
  // let heuristic_board = CombinedHeuristic::from_move_board(board, goal);
  let heuristic_board = GroupMinMovesBoard::from_move_board(board, goal);
  // let heuristic_board = ExpensiveCrawlsBoard::from_move_board(board, goal);
  // let heuristic_board = MinAssistsBoard::from_move_board(board, goal);

  // let mut queue = BucketingMonotonicPriorityQueue::with_capacities(256, 1024);
  let mut queue = BucketingPriorityQueue::with_capacities(256, 1024);
  let mut visited: HashMap<u32, VisitedData> = HashMap::with_capacity(1024);

  queue.push(
    QueueData {
      actor_squares,
      depth: 0,
    },
    heuristic_board.get_heuristic(actor_squares) as usize,
  );

  while let Some(queue_data) = queue.pop() {
    let QueueData {
      actor_squares,
      depth,
    } = queue_data;

    if depth as usize > max_depth {
      return None;
    }

    if actor_squares.0.contains(&goal) {
      let mut solution_steps = Vec::with_capacity(depth as usize);
      let mut current_actor_squares = actor_squares;
      for _ in 0..depth {
        let visited_key = current_actor_squares.as_sorted_u32();
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
      return Some(Solution(solution_steps));
    }

    for actor_index in 0..actor_squares.0.len() {
      let actor_square = actor_squares.0[actor_index];
      for direction in Direction::VARIANTS {
        let move_destination =
          board.get_move_destination(actor_square, actor_squares, direction);
        if move_destination == actor_square {
          // No change in position, skip
          continue;
        }

        let new_depth = depth + 1;

        let mut new_actor_squares = actor_squares;
        new_actor_squares.0[actor_index] = move_destination;

        let visited_key = new_actor_squares.as_sorted_u32();
        let prospective_value = VisitedData {
          depth: new_depth,
          parent: actor_squares,
        };
        let visited_entry = visited.entry(visited_key);
        let skippable = match visited_entry {
          Entry::Occupied(mut entry) => {
            let existing: &mut VisitedData = entry.get_mut();
            if existing.depth <= prospective_value.depth {
              true
            } else {
              *existing = prospective_value;
              false
            }
          }
          Entry::Vacant(entry) => {
            entry.insert(prospective_value);
            false
          }
        };

        if skippable {
          continue;
        }

        queue.push(
          QueueData {
            actor_squares: new_actor_squares,
            depth: new_depth,
          },
          new_depth as usize
            + heuristic_board.get_heuristic(actor_squares) as usize,
        );
      }
    }
  }

  None
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
