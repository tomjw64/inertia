use std::cmp::max;

use crate::mechanics::ActorSquares;
use crate::mechanics::MoveBoard;
use crate::mechanics::Square;

use super::get_min;
use super::get_min_two;
use super::group_min_moves_board;
use super::min_assists_board;
use super::min_moves_board;
use super::GroupMinMovesBoard;
use super::Heuristic;
use super::HeuristicValue;
use super::MinAssistsBoard;
use super::MinMovesBoard;

pub struct CombinedHeuristic {
  min_assists_board: MinAssistsBoard,
  min_moves_board: MinMovesBoard,
  group_min_moves_board: GroupMinMovesBoard,
  min_moves_min_required_assist: HeuristicValue,
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
  return HeuristicValue::MAX;
}

impl CombinedHeuristic {
  pub fn from_move_board(board: &MoveBoard, goal: Square) -> CombinedHeuristic {
    let min_assists_board = MinAssistsBoard::from_move_board(board, goal);
    let min_moves_board = MinMovesBoard::from_move_board(board, goal);
    let group_min_moves_board =
      GroupMinMovesBoard::from_move_board(board, goal);
    let min_moves_min_required_assist =
      get_min_moves_min_required_assist(&min_assists_board, &min_moves_board);
    let max_group_min_moves_to_min_required_assist =
      get_max_group_min_moves_to_min_required_assist(
        &min_moves_min_required_assist,
      );
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

    max(heuristic_from_forced_assist, heuristic_from_group_min_moves)
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
