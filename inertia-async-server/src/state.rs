use inertia_core::mechanics::WalledBoardPositionGenerator;
use inertia_core::message::ToClientMessage;
use inertia_core::state::data::RoomId;
use inertia_core::state::data::RoomState;
use inertia_core::state::event::apply_event::RoomEvent;
use inertia_core::state::event::result::EventError;
use inertia_core::state::event::result::EventResult;
use std::collections::HashMap;
use std::mem;
use thiserror::Error;
use tokio::sync::broadcast;
use tokio::sync::Mutex;
use tokio::sync::RwLock;

pub struct RoomUtils {
  pub channel: broadcast::Sender<String>,
}

pub struct Room {
  pub utils: RoomUtils,
  pub state: Mutex<RoomState>,
}

impl Room {
  pub fn new<T: WalledBoardPositionGenerator + 'static>(
    room_id: RoomId,
    generator: T,
  ) -> Self {
    Room {
      utils: RoomUtils {
        channel: broadcast::channel(16).0,
      },
      state: Mutex::new(RoomState::initial(room_id, generator)),
    }
  }
}

pub struct AppState {
  pub rooms: RwLock<HashMap<RoomId, Room>>,
}

#[derive(Error, Debug)]
pub enum BroadcastError {
  #[error(transparent)]
  NoRoomExists(#[from] NoRoomExistsError),
  #[error(transparent)]
  SendFailed(broadcast::error::SendError<String>),
}

#[derive(Error, Debug)]
pub enum ApplyEventError {
  #[error(transparent)]
  NoRoomExists(#[from] NoRoomExistsError),
  #[error(transparent)]
  ApplyRoomEventError(#[from] EventError),
}

#[derive(Error, Debug)]
#[error("Room {} does not exist", self.0.0)]
pub struct NoRoomExistsError(RoomId);

impl AppState {
  async fn with_room<F, T, E>(&self, room_id: RoomId, f: F) -> Result<T, E>
  where
    F: FnOnce(&RoomUtils, &mut RoomState) -> Result<T, E>,
    E: From<NoRoomExistsError>,
  {
    let rooms = self.rooms.read().await;
    let room = rooms.get(&room_id).ok_or(NoRoomExistsError(room_id))?;
    let mut state = room.state.lock().await;

    f(&room.utils, &mut state)
  }

  pub async fn cleanup_room(&self, room_id: RoomId) {
    let should_remove = self
      .with_room(room_id, |_, state| {
        Ok::<_, NoRoomExistsError>(matches!(*state, RoomState::Closed))
      })
      .await
      .unwrap_or(false);
    if should_remove {
      self.rooms.write().await.remove(&room_id);
    }
  }

  pub async fn get_channel_pair(
    &self,
    room_id: RoomId,
  ) -> Result<
    (broadcast::Sender<String>, broadcast::Receiver<String>),
    NoRoomExistsError,
  > {
    self
      .with_room(room_id, |utils, _| {
        Ok((utils.channel.clone(), utils.channel.subscribe()))
      })
      .await
  }

  pub async fn broadcast_room(
    &self,
    room_id: RoomId,
  ) -> Result<(), BroadcastError> {
    self
      .with_room(room_id, |utils, state| {
        let msg = ToClientMessage::RoomUpdate(&*state);

        utils
          .channel
          .send(serde_json::to_string(&msg).unwrap())
          .map_err(BroadcastError::SendFailed)?;

        Ok(())
      })
      .await
  }

  pub async fn apply_event(
    &self,
    room_id: RoomId,
    event: RoomEvent,
  ) -> Result<(), ApplyEventError> {
    self
      .with_room(room_id, |_, state| {
        let working_state = mem::replace(state, RoomState::None);
        let EventResult {
          result: next_state,
          error,
        } = working_state.apply(event);
        *state = next_state;
        error.map(Err).unwrap_or(Ok(()))?;
        Ok(())
      })
      .await
  }
}
