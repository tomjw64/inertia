use std::collections::HashMap;

use crate::state::PlayerId;
use crate::state::PlayerName;
use crate::state::PlayerReconnectKey;
use crate::state::PlayerScore;
use crate::state::RoomData;
use serde::Serialize;
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize)]
pub enum ToClientMessage {
  ErrorMessage(String),
  RoomUpdate(RoomData),
}
