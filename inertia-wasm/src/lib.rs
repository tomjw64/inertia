mod js_ffi;
mod log;
mod utils;

use inertia_core::mechanics::BlockBoard;
use inertia_core::mechanics::Direction;
use inertia_core::mechanics::ExpandedBitBoard;
use inertia_core::mechanics::Square;
use inertia_core::mechanics::WalledBoard;

use inertia_core::mechanics::WalledBoardPosition;
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
  return match actor {
    0..=3 => {
      let WalledBoardPosition {
        walled_board,
        actor_squares,
        ..
      } = board_position.0;
      let direction = direction.0;

      let expanded_bitboard = BlockBoard::from(&walled_board)
        .get_movement_ray(
          actor_squares.0[actor],
          actor_squares.as_bitboard(),
          direction,
        )
        .to_expanded();
      ExpandedBitBoardWrapper(expanded_bitboard)
    }
    _ => ExpandedBitBoardWrapper([false; 256]),
  };
}

#[wasm_bindgen]
pub fn get_movement_for_actor(
  board_position: WalledBoardPositionWrapper,
  actor: usize,
  direction: DirectionWrapper,
) -> SquareWrapper {
  return match actor {
    0..=3 => {
      let WalledBoardPosition {
        walled_board,
        actor_squares,
        ..
      } = board_position.0;
      let direction = direction.0;

      let square = BlockBoard::from(&walled_board).get_move_destination(
        actor_squares.0[actor],
        actor_squares.as_bitboard(),
        direction,
      );
      SquareWrapper(square)
    }
    _ => SquareWrapper(Square::new(0)),
  };
}
