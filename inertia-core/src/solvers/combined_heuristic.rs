use std::cmp::max;

use itertools::Itertools;

use crate::mechanics::ActorSquares;
use crate::mechanics::Direction;
use crate::mechanics::MoveBoard;
use crate::mechanics::Square;

use super::get_max;
use super::get_min_two;
use super::GroupMinMovesBoard;
use super::Heuristic;
use super::HeuristicValue;
use super::MinAssistsBoard;
use super::MinMovesBoard;

// Assist board notes:
// 2222 -> 2221 -> 2211 -> 2210 (3 moves, definitely the fastest, need two on N to move to N-1)
// e.g http://inertia.localhost:8080/edit?position=AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABQAFAAUABQAFAAUAAAAAAAAAAAAAAAAAAAAAAAAAAAAX1D_8W
// 3322 ->
//  - 3321
//  -

// IMPORTANT FOR PROOF:
// 2 actors: http://inertia.localhost:8080/edit?position=AACAAAAAgAAAAAAAAAABAAEAAAAAAAAAAAAAAAAAAABAAQAAAAAAAAAAAAAAAAAAAAAIAAAAAAAAAAAAAAAAAAqA-nAo
// 3 actors: http://inertia.localhost:8080/edit?position=AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQP8mJykY
// Ugh, tough math, but best guess:
// P(...) is the number of moves to reach zero given min_assist values
// P(N, N) = 6N - 5
// P(N, N, N) = 6N - 8
// P(N, N, N, N) = 6N - 9
//
// P(N, N + 1) = 6N - 2
// P(N, M) = M - N - 1 + P(M, M - 1) = M - N - 1 + 6(M-1) - 2 = 7M - N - 9
// P(N, M, Q)
pub struct CombinedHeuristic {
  min_assists_board: MinAssistsBoard,
  min_moves_board: MinMovesBoard,
  group_min_moves_board: GroupMinMovesBoard,
  min_moves_min_required_assist: HeuristicValue,
}

pub fn get_min_num_actors_for_assist_value(
  move_board: &MoveBoard,
  min_assists_board: &MinAssistsBoard,
) -> [u8; 256] {
  let mut must_leave_actor_behind_for_assist_value = [true; 256];

  let mut horizontal_seen = [false; 256];
  for idx in 0..=255 {
    if horizontal_seen[idx as usize] {
      continue;
    }
    let span_squares = move_board
      .get_unimpeded_movement_ray_squares(Square(idx), Direction::Right);
    for &Square(seen) in span_squares.iter() {
      horizontal_seen[seen as usize] = true;
    }
    let span_squares_min_assists = span_squares
      .iter()
      .map(|&s| min_assists_board.get(s))
      .collect::<Vec<_>>();
    let span_max = *span_squares_min_assists
      .iter()
      .max()
      .expect("Always at least one value");
    let span_min = *span_squares_min_assists
      .iter()
      .min()
      .expect("Always at least one value");
    if span_min == span_max {
      continue;
    }
    let has_two_consectutive_span_min = span_squares_min_assists
      .iter()
      .cloned()
      .tuple_windows::<(u8, u8)>()
      .filter(|&tuple| tuple == (span_min, span_min))
      .count()
      > 0;
    if has_two_consectutive_span_min {
      must_leave_actor_behind_for_assist_value[span_min as usize] = false;
    }
  }

  dbg!(must_leave_actor_behind_for_assist_value);

  let mut vertical_seen = [false; 256];
  for idx in 0..=255 {
    if vertical_seen[idx as usize] {
      continue;
    }
    let span_squares = move_board
      .get_unimpeded_movement_ray_squares(Square(idx), Direction::Down);
    for &Square(seen) in span_squares.iter() {
      vertical_seen[seen as usize] = true;
    }
    let span_squares_min_assists = span_squares
      .iter()
      .map(|&s| min_assists_board.get(s))
      .collect::<Vec<_>>();
    let span_max = *span_squares_min_assists
      .iter()
      .max()
      .expect("Always at least one value");
    let span_min = *span_squares_min_assists
      .iter()
      .min()
      .expect("Always at least one value");
    if span_min == span_max {
      continue;
    }
    let has_two_consectutive_span_min = span_squares_min_assists
      .iter()
      .cloned()
      .tuple_windows::<(u8, u8)>()
      .filter(|&tuple| tuple == (span_min, span_min))
      .count()
      > 1;
    if has_two_consectutive_span_min {
      must_leave_actor_behind_for_assist_value[span_min as usize] = false;
    }
  }

  let mut num_actors_needed_at_assist_value = [0; 256];
  num_actors_needed_at_assist_value[0] = 1; // Can't solve the puzzle without getting one to zero assists
  num_actors_needed_at_assist_value[1] = 2; // Exactly enough to transfer a single actor to 0, but you never need more.
  let mut current_needed: u8 = 2;
  for idx in 2..=255 {
    if must_leave_actor_behind_for_assist_value[idx - 1] {
      current_needed = current_needed.saturating_add(1);
    }
    num_actors_needed_at_assist_value[idx] = current_needed;
  }
  num_actors_needed_at_assist_value
}

fn get_min_moves_min_required_assist(
  min_assists_board: &MinAssistsBoard,
  min_moves_board: &MinMovesBoard,
) -> HeuristicValue {
  let mut val_to_all_require_assist = [true; 256];
  for idx in 0..=255 {
    let val = min_moves_board.get(Square(idx));
    let no_assist_required = min_assists_board.get(Square(idx)) == 0;
    if no_assist_required {
      val_to_all_require_assist[val as usize] = false;
    }
  }
  for idx in 0u8..=255 {
    if val_to_all_require_assist[idx as usize] {
      return idx;
    }
  }
  HeuristicValue::MAX
}

impl CombinedHeuristic {
  pub fn from_move_board(board: &MoveBoard, goal: Square) -> CombinedHeuristic {
    let min_assists_board = MinAssistsBoard::from_move_board(board, goal);
    let min_moves_board = MinMovesBoard::from_move_board(board, goal);
    let group_min_moves_board =
      GroupMinMovesBoard::from_move_board(board, goal);
    let min_moves_min_required_assist =
      get_min_moves_min_required_assist(&min_assists_board, &min_moves_board);
    CombinedHeuristic {
      min_assists_board,
      min_moves_board,
      group_min_moves_board,
      min_moves_min_required_assist,
    }
  }
}

impl Heuristic for CombinedHeuristic {
  fn get_heuristic(&self, actor_squares: ActorSquares) -> HeuristicValue {
    // What is this?
    // IMPORTANT FOR FUTURE PROOF:
    // Theory: it's not possible to decrease the min_assists_board heuristic by
    // 1 in less than 6 moves on average. This is based on the idea that, in the
    // min_assists_board, every span (i.e. area between two walls) can be
    // defined by it's min and max values (which never differ by more than one).
    // When actors assist one another to decrase the their min_assists values,
    // if both start on an N/N-1 span, ready to assist each other, they must
    // both start on N on opposite sides of an N-1 square, one actor must assist
    // the other to an N-1 square, then the N-1 actor must assist the N actor
    // itself to an N-1 square, again on an N/N-1, and then they must position
    // themselves to be ready to assist each other on a N-1/N-2 span (to
    // complete the cycle, which is important for determining the average - the
    // cycle being "ready to assist on N to ready to assist on N-1"). See:
    // http://inertia.localhost:8080/edit?position=AACAAAAAgAAAAAAAAAABAAEAAAAAAAAAAAAAAAAAAABAAQAAAAAAAAAAAAAAAAAAAAAIAAAAAAAAAAAAAAAAAAqA-nAo
    // for an example of two actors assisting each other in such a cycle. (The
    // solution takes 7 moves, 6 for a cycle, and then the puzzle can be
    // completed before the next cycle). It is unclear how many times it is actually
    // possible a cycle of 6 can repeat, just given constraints on board
    // geometry.
    // It is also not clear if 3 actors can cycle more quickly on average.
    // Intuitively, it would seem improbable, since it would leave 2 moves per
    // actor to both assist on an N/N-1 span, and then be ready to do the same
    // process again on a N-1/N-2 span, but I don't have any real for basically
    // any of this.
    // Very difficult to show, but best guess:
    // P(...) is the number of moves to reach zero given min_assist values
    // P(N, N) = 6N - 5                                 // Somewhat confident
    // P(N + 1, N) = 6N - 2                             // Somewhat confident
    // P(M, N) = M - N - 1 + P(M, M - 1) = 7M - N - 9   // Somewhat confident (N moves backwards to M - 1 first)
    // P(N, N, N) = 6N - 8                              // Much harder to prove
    // P(N, N, N, N) = 6N - 9                           // Much harder to prove
    // Some boards used to come up with these estimations
    // 3 actors:
    // http://inertia.localhost:8080/edit?position=AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQP8mJykY
    // 4 actors:
    // http://inertia.localhost:8080/edit?position=AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAUnJQcW
    // Since the lowest heuristic w/ all 4 actors on N is 6N-9, let's use that.
    // In the future, perhaps we can use the minimum two or fill out this
    // function with 4 variables.
    // let experimental_min_assists_heuristic =
    //   (self.min_assists_board.get_heuristic(actor_squares) * 6)
    //     .saturating_sub(9);
    // Uh oh, not admissible. 6 * 7 - 9 = 33, but this is solvable in 30 moves
    // http://inertia.localhost:8080/explore?position=CAAgACAAgQCBAAUCBQIVCBUIVSBVIFUBVQFVBVUFVRUIACAAIACBAIEABQIFAhUIFQhVIFUgVQFVAVUFVQVVFRJSIgLu&solution=Optimal%3AHgC5Mf2hKb2PCsPbHjn8sJ0

    let min_assists_heuristic =
      self.min_assists_board.get_heuristic(actor_squares);
    let heuristic_from_forced_assist = if min_assists_heuristic > 0 {
      // At least one assist is required to complete the puzzle. If at least
      // one assist is require to complete the puzzle, it is also true that
      // the closest actor requires at least one assist to reach the goal.
      let min_moves_board_min_two = get_min_two(
        actor_squares
          .0
          .map(|square| self.min_moves_board.squares[square.0 as usize]),
      );

      // See below comments for why we check this condition. If false, we can
      // get an even higher heuristic.
      if min_moves_board_min_two[0] <= self.min_moves_min_required_assist {
        // Given: All squares on min move board are no more than 1 different from
        // all other squares they could move to in one move under ANY
        // circumstance. Put another way, no actor on a square with value N can
        // ever move to a square with value M<=N-2 in a single move, even under
        // ideal assisted conditions.
        //
        // Optimization of heuristic for cases where the closest actor cannot
        // reach the goal unassisted:
        //
        // By definition of the case, at least one assisted move must be made to
        // reach the goal.
        //
        // Let's say our closest actor is on a square of value P and our second
        // closest is on a square of value Q.
        //
        // If we make our assisted move to make the value of the closest actor
        // P-1, then we had to have put a assisting actor on a square with at most
        // P. It will take at least Q - P moves to get our assisting actor to a
        // square of value P. Then we perform the assist. This takes 1 move, and
        // then our closest actor is on a square of value P - 1. Thus our
        // heuristic at the starting position can be (Q - P) + 1 + (P - 1) = Q.
        //
        // If we make an assisted move to make the value of the closest actor P,
        // then we could have put an assisting actor on a square with at most P +
        // 1. It will take at least Q - (P + 1) moves to get our assisting actor
        // to a square of value P + 1. Then we perform the assist. This takes 1
        // move, and then our closest actor is on a square of value P. Thus our
        // heuristic at the starting position can be (Q - (P + 1)) + 1 + P = Q
        //
        // If we move the closest actor to a square with value P + 1, then all the
        // same apply, and this doesn't decrease our heuristic in an unexpected
        // way. Using the same logic, it will from that point take at least
        // either:
        //   - (Q - (P + 1)) + 1 + (P + 1 - 1) = Q or
        //   - (Q - ((P+1) + 1)) + 1 + (P + 1) = Q moves to finish the puzzle.
        min_moves_board_min_two[1]
      } else {
        // Determine the lowest MinMovesBoard value for which all squares
        // having that value require at least one assist to reach the goal
        // square. Let's call this value L. If our closest and second closest
        // actors are on squares of values P > L and Q > P > L, respectively,
        // then we can yield an even higher heuristic than above: at least (P
        // - L) + (Q - L) + L = P + Q - L. That is, at least P - L moves to
        // get the closest actor to a square of value L, then an additional Q
        // moves, based on the same logic in the above comment. Attempting to
        // transfer earlier does not escape this, because if all squares with
        // value L require at least one assist to reach the goal, then all
        // squares with values greater than L also must require at least one
        // assist to reach the goal. This is, again, due to the fact no actor
        // on a square with value N can ever move to a square with value
        // M<=N-2 in a single move, even under ideal assisted conditions. In
        // fact, we even know that an assist MUST occur with two actors on
        // squares with values L or less in order for the puzzle to be solved.
        // "or less" because there may be squares of e.g. value L - 1 still
        // requiring at least one assist that can be reached without
        // assistance, and then the assist may occur using that square.
        // However, if we know there is a value of square for which there are
        // no squares with that value or lower that require any assist, we may
        // be able to tighten this constraint (not that it would particularly
        // help).
        min_moves_board_min_two[0] + min_moves_board_min_two[1]
          - self.min_moves_min_required_assist
      }
    } else {
      HeuristicValue::MIN
    };

    // The result of GroupMinMovesBoard::get_heuristic is always greater than
    // the result of MinMovesBoard::get_heuristic which is itself always greater
    // than the result of MinAssistsBoard::get_heuristic.
    let heuristic_from_group_min_moves =
      self.group_min_moves_board.get_heuristic(actor_squares);

    get_max([
      // experimental_min_assists_heuristic, not admissible
      heuristic_from_forced_assist,
      heuristic_from_group_min_moves,
    ])
  }

  fn get_heuristic_for_target_actor(
    &self,
    actor_squares: crate::mechanics::ActorSquares,
    actor_index: usize,
  ) -> HeuristicValue {
    // The result of MinMovesBoard::get_heuristic_for_target_actor is always
    // greater than the result of
    // MinAssistsBoard::get_heuristic_for_target_actor. GroupMinMovesBoard does
    // not provide an optimization for a target actor. The result of
    // GroupMinMovesBoard::get_heuristic is not necessarily always greater than
    // the result of MinMovesBoard::get_heuristic_for_target_actor.
    max(
      self.group_min_moves_board.get_heuristic(actor_squares),
      self
        .min_moves_board
        .get_heuristic_for_target_actor(actor_squares, actor_index),
    )
  }
}
