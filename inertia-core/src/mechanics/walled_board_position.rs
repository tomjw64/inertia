use serde::Deserialize;
use serde::Serialize;
use typeshare::typeshare;

use crate::mechanics::ActorSquares;
use crate::mechanics::Square;
use crate::mechanics::WalledBoard;

#[typeshare]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct WalledBoardPosition {
  pub walled_board: WalledBoard,
  pub actor_squares: ActorSquares,
  pub goal: Square,
}

pub trait WalledBoardPositionGenerator {
  fn generate_position(&self) -> WalledBoardPosition;
}
