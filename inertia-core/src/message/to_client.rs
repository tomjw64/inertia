use crate::state::data::RoomState;
use serde::Serialize;
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize)]
#[serde(tag = "type", content = "content")]
pub enum ToClientMessage<'a> {
  RoomUpdate(&'a RoomState),
}
