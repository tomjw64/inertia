use thiserror::Error;

use crate::state::data::RoomState;

use super::apply_event::RoomEvent;
use super::connect::ConnectError;
use super::update_solution::UpdateSolutionError;

#[derive(Error, Debug)]
pub enum EventError {
  #[error("State {:?} is incompatible with event {}", .0, .1)]
  IncompatibleState(String, RoomEvent),
  #[error(transparent)]
  ConnectError(#[from] ConnectError),
  #[error(transparent)]
  UpdateSolutionError(#[from] UpdateSolutionError),
}

pub struct EventResult {
  pub result: RoomState,
  pub error: Option<EventError>,
}

impl EventResult {
  pub fn ok(result: RoomState) -> Self {
    EventResult {
      result,
      error: None,
    }
  }
  pub fn err<E: Into<EventError>>(result: RoomState, error: E) -> Self {
    EventResult {
      result,
      error: Some(error.into()),
    }
  }
}
