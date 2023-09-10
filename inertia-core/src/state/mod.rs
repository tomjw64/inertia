use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use typeshare::typeshare;

use crate::mechanics::WalledBoardPosition;
use crate::solvers::SolutionStep;

#[typeshare(serialized_as = "number")]
#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone)]
pub struct PlayerId(pub usize);

#[typeshare(serialized_as = "number")]
#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone)]
pub struct RoomId(pub usize);

#[typeshare]
#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct PlayerName(pub String);

#[typeshare(serialized_as = "number")]
#[derive(Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub struct PlayerScore(pub usize);

#[typeshare(serialized_as = "number")]
#[derive(Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
pub struct PlayerReconnectKey(pub usize);

#[typeshare(serialized_as = "number")]
#[derive(Serialize, Deserialize)]
pub struct PlayerBid(usize);

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct RoomData {
  pub room_id: RoomId,
  pub players: HashMap<PlayerId, PlayerName>,
  #[serde(skip)]
  pub player_reconnect_keys: HashMap<PlayerId, PlayerReconnectKey>,
  pub players_connected: HashMap<PlayerId, bool>,
  pub player_scores: HashMap<PlayerId, PlayerScore>,
  #[typeshare(typescript(type = "number"))]
  pub round_number: usize,
  pub state: RoomState,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum RoomState {
  Lobby,
  RoundSummary,
  RoundStart(RoundStart),
  RoundBidding(RoundBidding),
  RoundSolving(RoundSolving),
}

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct RoundStart {
  pub board: WalledBoardPosition,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct RoundBidding {
  pub board: WalledBoardPosition,
  pub player_bids: HashMap<PlayerId, PlayerBid>,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct RoundSolving {
  pub board: WalledBoardPosition,
  pub player_bids: HashMap<PlayerId, PlayerBid>,
  pub solver: PlayerId,
  pub solution: Vec<SolutionStep>,
}

// #[typeshare]
// #[derive(Serialize, Deserialize)]
// pub struct PlayerName {
//   pub id: PlayerId,
//   pub name: String,
// }

// #[typeshare]
// #[derive(Serialize, Deserialize)]
// pub struct PlayerScore {
//   pub id: PlayerId,
//   #[typeshare(typescript(type = "number"))]
//   pub score: usize,
// }

// #[typeshare]
// #[derive(Serialize, Deserialize)]
// pub struct PlayerBid {
//   pub player: PlayerId,
//   pub bid: Option<Bid>,
// }
