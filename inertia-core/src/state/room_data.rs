use serde::Deserialize;
use serde::Serialize;
use typeshare::typeshare;

use crate::mechanics::WalledBoardPosition;
use crate::solvers::SolutionStep;

#[typeshare(serialized_as = "number")]
#[derive(Serialize, Deserialize)]
pub struct PlayerId(usize);

#[typeshare(serialized_as = "number")]
#[derive(Serialize, Deserialize)]
pub struct RoomId(usize);

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct RoomData {
  room_id: RoomId,
  #[typeshare(typescript(type = "number"))]
  updated_at_epoch_millis: usize,
  players: Vec<PlayerName>,
  player_scores: Vec<PlayerScore>,
  #[typeshare(typescript(type = "number"))]
  round_number: usize,
  #[typeshare(typescript(type = "number"))]
  data_version: usize,
  state: RoomState,
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
  board: WalledBoardPosition,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct RoundBidding {
  board: WalledBoardPosition,
  player_bids: Vec<PlayerBid>,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct RoundSolving {
  board: WalledBoardPosition,
  player_bids: Vec<PlayerBid>,
  solver: PlayerId,
  solution: Vec<SolutionStep>,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct PlayerName {
  id: PlayerId,
  name: String,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct PlayerScore {
  id: PlayerId,
  #[typeshare(typescript(type = "number"))]
  score: usize,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct PlayerBid {
  player: PlayerId,
  bid: Option<Bid>,
}

#[typeshare(serialized_as = "number")]
#[derive(Serialize, Deserialize)]
pub struct Bid(usize);
