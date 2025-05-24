pub mod js_ffi;
pub mod log;

use crate::mechanics::ActorSquares;
use crate::mechanics::B64EncodedCompressedPosition;
use crate::mechanics::CompressedPosition;
use crate::mechanics::Direction;
use crate::mechanics::ExpandedBitBoard;
use crate::mechanics::MoveBoard;
use crate::mechanics::Position;
use crate::mechanics::Square;
use crate::solvers::astar;
use crate::solvers::difficulty::get_solution_difficulty;
use crate::solvers::B64EncodedCompressedSolution;
use crate::solvers::CompressedSolution;
use crate::solvers::Difficulty;
use crate::solvers::GroupMinMovesBoard;
use crate::solvers::HeuristicValue;
use crate::solvers::MinAssistsBoard;
use crate::solvers::MinCrawlsBoard;
use crate::solvers::MinMovesBoard;
use crate::solvers::Solution;
use crate::state::data::PlayerBids;
use serde::Deserialize;
use serde::Serialize;
use serde_with::serde_as;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ExpandedBitBoardWrapper(
  #[serde_as(as = "[_; 256]")] ExpandedBitBoard,
);

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct MetaBoardWrapper {
  #[serde_as(as = "[_; 256]")]
  squares: [HeuristicValue; 256],
}

#[wasm_bindgen]
pub fn set_panic_hook() {
  #[cfg(feature = "console_error_panic_hook")]
  console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn get_movement_ray_for_actor(
  board_position: Position,
  actor: usize,
  direction: Direction,
) -> ExpandedBitBoardWrapper {
  match actor {
    0..=3 => {
      let Position {
        walled_board,
        actor_squares,
        ..
      } = board_position;

      let expanded_bitboard = MoveBoard::from(&walled_board)
        .get_movement_ray(actor_squares.0[actor], actor_squares, direction)
        .to_expanded();
      ExpandedBitBoardWrapper(expanded_bitboard)
    }
    _ => ExpandedBitBoardWrapper([false; 256]),
  }
}

#[wasm_bindgen]
pub fn get_movement_for_actor(
  board_position: Position,
  actor: usize,
  direction: Direction,
) -> Square {
  match actor {
    0..=3 => {
      let Position {
        walled_board,
        actor_squares,
        ..
      } = board_position;

      let square = MoveBoard::from(&walled_board).get_move_destination(
        actor_squares.0[actor],
        actor_squares,
        direction,
      );
      square
    }
    _ => Square(0),
  }
}

#[wasm_bindgen]
pub fn apply_solution(
  board_position: Position,
  solution: Solution,
) -> ActorSquares {
  board_position.apply_solution(&solution)
}

#[wasm_bindgen]
pub fn is_solution(board_position: Position, solution: Solution) -> bool {
  board_position.is_solution(&solution)
}

#[wasm_bindgen]
pub fn get_difficulty(solution: Solution) -> Difficulty {
  get_solution_difficulty(&solution)
}

#[wasm_bindgen]
pub fn get_next_solver(player_bids: PlayerBids) -> Option<u32> {
  player_bids.get_next_solver().map(|id| id.0)
}

#[wasm_bindgen]
pub fn decode_position(
  bytes: B64EncodedCompressedPosition,
) -> Option<Position> {
  Some(Position::try_from(CompressedPosition::try_from(bytes).ok()?).ok()?)
}

#[wasm_bindgen]
pub fn encode_position(position: Position) -> B64EncodedCompressedPosition {
  B64EncodedCompressedPosition::from(CompressedPosition::from(position))
}

#[wasm_bindgen]
pub fn decode_solution(
  bytes: B64EncodedCompressedSolution,
) -> Option<Solution> {
  Some(Solution::try_from(CompressedSolution::try_from(bytes).ok()?).ok()?)
}

#[wasm_bindgen]
pub fn encode_solution(solution: Solution) -> B64EncodedCompressedSolution {
  B64EncodedCompressedSolution::from(CompressedSolution::from(solution))
}

#[wasm_bindgen]
pub fn get_group_min_moves_board(board_position: Position) -> MetaBoardWrapper {
  let Position {
    walled_board, goal, ..
  } = board_position;
  let board = MoveBoard::from(&walled_board);
  let GroupMinMovesBoard { squares } =
    GroupMinMovesBoard::from_move_board(&board, goal);
  MetaBoardWrapper { squares }
}

#[wasm_bindgen]
pub fn get_min_moves_board(board_position: Position) -> MetaBoardWrapper {
  let Position {
    walled_board, goal, ..
  } = board_position;
  let board = MoveBoard::from(&walled_board);
  let MinMovesBoard { squares } = MinMovesBoard::from_move_board(&board, goal);
  MetaBoardWrapper { squares }
}

#[wasm_bindgen]
pub fn get_min_assists_board(board_position: Position) -> MetaBoardWrapper {
  let Position {
    walled_board, goal, ..
  } = board_position;
  let board = MoveBoard::from(&walled_board);
  let MinAssistsBoard { squares } =
    MinAssistsBoard::from_move_board(&board, goal);
  MetaBoardWrapper { squares }
}

#[wasm_bindgen]
pub fn get_min_crawls_board(board_position: Position) -> MetaBoardWrapper {
  let Position {
    walled_board, goal, ..
  } = board_position;
  let board = MoveBoard::from(&walled_board);
  let MinCrawlsBoard { squares } =
    MinCrawlsBoard::from_move_board(&board, goal);
  MetaBoardWrapper { squares }
}

#[wasm_bindgen]
pub fn solve(board_position: Position) -> Option<Solution> {
  let Position {
    walled_board,
    actor_squares,
    goal,
  } = board_position;
  let board = MoveBoard::from(&walled_board);

  astar::solve(&board, goal, actor_squares, 255)
}
