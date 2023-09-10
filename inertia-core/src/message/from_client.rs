use crate::state::PlayerId;
use crate::state::PlayerName;
use crate::state::PlayerReconnectKey;
use crate::state::RoomId;
use serde::Deserialize;
use typeshare::typeshare;

#[typeshare]
#[derive(Deserialize)]
pub enum FromClientMessage {
  Rename(RenameMessage),
  Join(JoinMessage),
  StartGame,
}

#[typeshare]
#[derive(Deserialize)]
pub struct RenameMessage {
  pub player_name: PlayerName,
}

#[typeshare]
#[derive(Deserialize)]
pub struct JoinMessage {
  pub player_name: PlayerName,
  pub player_id: PlayerId,
  pub player_reconnect_key: PlayerReconnectKey,
  pub room_id: RoomId,
}
