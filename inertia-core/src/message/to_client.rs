use crate::state::data::RoomState;
use serde::Serialize;
use typeshare::typeshare;

#[typeshare]
#[derive(Serialize, Clone, Debug)]
#[serde(tag = "type", content = "content")]
pub enum ToClientMessage {
  RoomUpdate(Box<RoomState>),
  CountdownUpdate(CountdownUpdateMessage),
  ExplicitPong,
}

#[typeshare]
#[derive(Serialize, Clone, Copy, Debug)]
pub struct CountdownUpdateMessage {
  #[typeshare(typescript(type = "number"))]
  pub server_time_left_millis: u128,
}
