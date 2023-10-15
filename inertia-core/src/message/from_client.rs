use crate::solvers::SolutionStep;
use crate::state::data::PlayerId;
use crate::state::data::PlayerName;
use crate::state::data::PlayerReconnectKey;
use crate::state::data::RoomId;
use serde::Deserialize;
use strum::Display;
use typeshare::typeshare;

#[typeshare]
#[derive(Deserialize, Debug, Display)]
#[serde(tag = "type", content = "content")]
pub enum FromClientMessage {
  ExplicitPing,
  Rename(RenameMessage),
  Join(JoinMessage),
  StartRound,
  Bid(BidMessage),
  LockInBid,
  UpdateSolution(UpdateSolutionMessage),
  GiveUpSolve,
}

#[typeshare]
#[derive(Deserialize, Debug)]
pub struct UpdateSolutionMessage {
  pub solution: Vec<SolutionStep>,
}

#[typeshare]
#[derive(Deserialize, Debug)]
pub struct BidMessage {
  #[typeshare(typescript(type = "number"))]
  pub bid_value: usize,
}

#[typeshare]
#[derive(Deserialize, Debug)]
pub struct RenameMessage {
  pub player_name: PlayerName,
}

#[typeshare]
#[derive(Deserialize, Debug)]
pub struct JoinMessage {
  pub player_name: PlayerName,
  pub player_id: PlayerId,
  pub player_reconnect_key: PlayerReconnectKey,
  pub room_id: RoomId,
}
