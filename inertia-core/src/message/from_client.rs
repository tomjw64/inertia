use crate::solvers::difficulty::Difficulty;
use crate::solvers::Solution;
use crate::state::data::PlayerId;
use crate::state::data::PlayerName;
use crate::state::data::PlayerReconnectKey;
use crate::state::data::RoomId;
use serde::Deserialize;
use strum::Display;

#[cfg(feature = "web")]
use {tsify::Tsify, wasm_bindgen::prelude::wasm_bindgen};

#[derive(Deserialize, Debug, Display)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(from_wasm_abi))]
#[serde(tag = "type", content = "content")]
pub enum FromClientMessage {
  ExplicitPing,
  Rename(RenameMessage),
  Join(JoinMessage),
  StartRound,
  Bid(BidMessage),
  ReadyBid,
  UnreadyBid,
  UpdateSolution(UpdateSolutionMessage),
  YieldSolve,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(from_wasm_abi))]
pub struct UpdateSolutionMessage {
  pub solution: Solution,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(from_wasm_abi))]
pub struct BidMessage {
  pub bid_value: u32,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(from_wasm_abi))]
pub struct RenameMessage {
  pub player_name: PlayerName,
}

#[derive(Deserialize, Debug)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(from_wasm_abi))]
pub struct JoinMessage {
  pub player_name: PlayerName,
  pub player_id: PlayerId,
  pub player_reconnect_key: PlayerReconnectKey,
  pub room_id: RoomId,
  pub min_difficulty: Option<Difficulty>,
  pub max_difficulty: Option<Difficulty>,
}
