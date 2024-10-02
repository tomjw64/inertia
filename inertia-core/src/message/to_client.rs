use crate::state::data::RoomState;
use serde::Serialize;

#[cfg(feature = "web")]
use {tsify::Tsify, wasm_bindgen::prelude::wasm_bindgen};

#[derive(Serialize, Clone, Debug)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(into_wasm_abi))]
#[serde(tag = "type", content = "content")]
pub enum ToClientMessage {
  RoomUpdate(Box<RoomState>),
  CountdownUpdate(CountdownUpdateMessage),
  ExplicitPong,
}

#[derive(Serialize, Clone, Copy, Debug)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(into_wasm_abi))]
pub struct CountdownUpdateMessage {
  pub server_time_left_millis: u128,
}
