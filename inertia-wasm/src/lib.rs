mod js_ffi;
mod log;
mod utils;

use std::convert::TryInto;

use base64::engine::general_purpose;
use base64::Engine;
use inertia_core::mechanics::ActorSquares;
use inertia_core::mechanics::Direction;
use inertia_core::mechanics::ExpandedBitBoard;
use inertia_core::mechanics::MoveBoard;
use inertia_core::mechanics::Square;
use inertia_core::mechanics::WalledBoard;

use inertia_core::mechanics::WalledBoardPosition;
use inertia_core::solvers::SolutionStep;
use inertia_core::state::data::PlayerBids;
use serde::Deserialize;
use serde::Serialize;
use serde_with::serde_as;
use tsify::Tsify;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::log::console_debug;
use crate::log::console_log;

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct WalledBoardWrapper(WalledBoard);

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct WalledBoardPositionWrapper(WalledBoardPosition);

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ExpandedBitBoardWrapper(
  #[serde_as(as = "[_; 256]")] ExpandedBitBoard,
);

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct DirectionWrapper(Direction);

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SquareWrapper(Square);

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ActorSquaresWrapper(ActorSquares);

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct PlayerBidsWrapper(PlayerBids);

#[derive(Debug, Serialize, Deserialize, Tsify)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct SolutionWrapper(Vec<SolutionStep>);

#[wasm_bindgen]
pub fn math() {
  console_debug!(1 + 2);
}

#[wasm_bindgen]
pub fn greet() {
  console_log!("Hello there, inertia-wasm!");
}

#[wasm_bindgen]
pub fn get_movement_ray_for_actor(
  board_position: WalledBoardPositionWrapper,
  actor: usize,
  direction: DirectionWrapper,
) -> ExpandedBitBoardWrapper {
  match actor {
    0..=3 => {
      let WalledBoardPosition {
        walled_board,
        actor_squares,
        ..
      } = board_position.0;
      let direction = direction.0;

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
  board_position: WalledBoardPositionWrapper,
  actor: usize,
  direction: DirectionWrapper,
) -> SquareWrapper {
  match actor {
    0..=3 => {
      let WalledBoardPosition {
        walled_board,
        actor_squares,
        ..
      } = board_position.0;
      let direction = direction.0;

      let square = MoveBoard::from(&walled_board).get_move_destination(
        actor_squares.0[actor],
        actor_squares,
        direction,
      );
      SquareWrapper(square)
    }
    _ => SquareWrapper(Square(0)),
  }
}

#[wasm_bindgen]
pub fn apply_solution(
  board_position: WalledBoardPositionWrapper,
  solution: SolutionWrapper,
) -> ActorSquaresWrapper {
  ActorSquaresWrapper(board_position.0.apply_solution(&solution.0))
}

#[wasm_bindgen]
pub fn get_next_solver(player_bids: PlayerBidsWrapper) -> Option<usize> {
  player_bids.0.get_next_solver().map(|id| id.0)
}

#[wasm_bindgen]
pub fn decode_position(bytes: String) -> Option<WalledBoardPositionWrapper> {
  let bytes = general_purpose::URL_SAFE_NO_PAD.decode(bytes).ok()?;
  Some(WalledBoardPositionWrapper(
    WalledBoardPosition::from_compressed_byte_array(
      &bytes[0..69].try_into().ok()?,
    ),
  ))
}

#[wasm_bindgen]
pub fn encode_position(position: WalledBoardPositionWrapper) -> String {
  general_purpose::URL_SAFE_NO_PAD.encode(position.0.to_compressed_byte_array())
}
