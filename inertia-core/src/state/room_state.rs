use serde::Deserialize;
use serde::Serialize;
use typeshare::typeshare;

use crate::mechanics::WalledBoardPosition;
use crate::solvers::SolutionStep;

#[typeshare(serialized_as = "number")]
#[derive(Serialize, Deserialize)]
struct PlayerId(usize);

#[typeshare(serialized_as = "number")]
#[derive(Serialize, Deserialize)]
struct RoomId(usize);

#[typeshare]
#[derive(Serialize, Deserialize)]
struct RoomData {
  room_id: RoomId,
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
enum RoomState {
  Lobby,
  RoundSummary,
  RoundStart(RoundStart),
  RoundBidding(RoundBidding),
  RoundSolving(RoundSolving),
}

#[typeshare]
#[derive(Serialize, Deserialize)]
struct RoundStart {
  board: WalledBoardPosition,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
struct RoundBidding {
  board: WalledBoardPosition,
  player_bids: Vec<PlayerBid>,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
struct RoundSolving {
  board: WalledBoardPosition,
  player_bids: Vec<PlayerBid>,
  solver: PlayerId,
  solution: Vec<SolutionStep>,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
struct PlayerName {
  id: PlayerId,
  name: String,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
struct PlayerScore {
  id: PlayerId,
  #[typeshare(typescript(type = "number"))]
  score: usize,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
struct PlayerBid {
  player: PlayerId,
  bid: Option<Bid>,
}

#[typeshare(serialized_as = "number")]
#[derive(Serialize, Deserialize)]
struct Bid(usize);
