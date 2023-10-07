use crate::state::data::PlayerId;
use crate::state::data::PlayerName;
use crate::state::data::PlayerReconnectKey;
use crate::state::data::RoomId;
use serde::Deserialize;
use typeshare::typeshare;

#[typeshare]
#[derive(Deserialize, Debug)]
#[serde(tag = "type", content = "content")]
pub enum FromClientMessage {
  Rename(RenameMessage),
  Join(JoinMessage),
  StartRound,
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
