use inertia_core::mechanics::WalledBoardPositionGenerator;
use inertia_core::message::CountdownUpdateMessage;
use inertia_core::message::ToClientMessage;
use inertia_core::state::data::RoomId;
use inertia_core::state::data::RoomState;
use inertia_core::state::event::apply_event::RoomEvent;
use inertia_core::state::event::result::EventError;
use inertia_core::state::event::result::EventResult;
use std::collections::HashMap;
use std::mem;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::broadcast;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
use tokio::time::Instant;

pub struct Countdown {
  pub task: JoinHandle<()>,
  pub stop: Instant,
}

pub struct RoomUtils {
  pub room_id: RoomId,
  pub broadcast_channel: broadcast::Sender<ToClientMessage>,
  pub countdown: Option<Countdown>,
}

pub struct Room {
  pub utils: RoomUtils,
  pub state: RoomState,
}

impl Room {
  pub fn new<G: WalledBoardPositionGenerator + 'static>(
    room_id: RoomId,
    generator: G,
  ) -> Self {
    Room {
      utils: RoomUtils {
        room_id,
        broadcast_channel: broadcast::channel(16).0,
        countdown: None,
      },
      state: RoomState::initial(room_id, generator),
    }
  }
}

#[derive(Clone)]
pub struct AppState {
  pub rooms: Arc<RwLock<HashMap<RoomId, RwLock<Room>>>>,
}

#[derive(Error, Debug)]
pub enum BroadcastError {
  #[error(transparent)]
  NoRoomExists(#[from] NoRoomExistsError),
  #[error("No countdown exists")]
  NoCountdownExists,
  #[error(transparent)]
  SendFailed(broadcast::error::SendError<ToClientMessage>),
}

#[derive(Error, Debug)]
pub enum ApplyEventError {
  #[error(transparent)]
  NoRoomExists(#[from] NoRoomExistsError),
  #[error(transparent)]
  ApplyRoomEventError(#[from] EventError),
  #[error("Validation for event failed.")]
  ValidationFailedError,
}

#[derive(Error, Debug)]
#[error("Room {} does not exist", self.0.0)]
pub struct NoRoomExistsError(RoomId);

impl AppState {
  async fn with_room_read<F, T, E>(&self, room_id: RoomId, f: F) -> Result<T, E>
  where
    F: FnOnce(&Room) -> Result<T, E>,
    E: From<NoRoomExistsError>,
  {
    let rooms = self.rooms.read().await;
    let room = rooms
      .get(&room_id)
      .ok_or(NoRoomExistsError(room_id))?
      .read()
      .await;
    f(&room)
  }

  async fn with_room_write<F, T, E>(
    &self,
    room_id: RoomId,
    f: F,
  ) -> Result<T, E>
  where
    F: FnOnce(&mut Room) -> Result<T, E>,
    E: From<NoRoomExistsError>,
  {
    let rooms = self.rooms.read().await;
    let mut room = rooms
      .get(&room_id)
      .ok_or(NoRoomExistsError(room_id))?
      .write()
      .await;

    f(&mut room)
  }

  pub async fn ensure_room_exists<G: WalledBoardPositionGenerator + 'static>(
    &self,
    room_id: RoomId,
    generator: G,
  ) {
    let should_create_room = self.rooms.read().await.get(&room_id).is_none();
    if should_create_room {
      self
        .rooms
        .write()
        .await
        .entry(room_id)
        .or_insert_with(|| RwLock::new(Room::new(room_id, generator)));
    }
  }

  pub async fn cleanup_room(&self, room_id: RoomId) {
    let should_remove = self
      .with_room_read(room_id, |room| {
        Ok::<_, NoRoomExistsError>(matches!(room.state, RoomState::Closed))
      })
      .await
      .unwrap_or(false);
    if should_remove {
      self.rooms.write().await.remove(&room_id);
    }
  }

  pub async fn get_broadcast_channel_pair(
    &self,
    room_id: RoomId,
  ) -> Result<
    (
      broadcast::Sender<ToClientMessage>,
      broadcast::Receiver<ToClientMessage>,
    ),
    NoRoomExistsError,
  > {
    self
      .with_room_read(room_id, |room| {
        Ok((
          room.utils.broadcast_channel.clone(),
          room.utils.broadcast_channel.subscribe(),
        ))
      })
      .await
  }

  pub async fn broadcast_room(
    &self,
    room_id: RoomId,
  ) -> Result<(), BroadcastError> {
    self
      .with_room_read(room_id, |room| {
        let msg = ToClientMessage::RoomUpdate(Box::new(room.state.clone()));

        room
          .utils
          .broadcast_channel
          .send(msg)
          .map_err(BroadcastError::SendFailed)?;

        Ok(())
      })
      .await
  }

  pub async fn broadcast_countdown(
    &self,
    room_id: RoomId,
  ) -> Result<(), BroadcastError> {
    self
      .with_room_read(room_id, |room| {
        let countdown_stop = room
          .utils
          .countdown
          .as_ref()
          .map(|c| c.stop)
          .ok_or(BroadcastError::NoCountdownExists)?;

        let now = Instant::now();
        let time_left = countdown_stop - now;

        let msg = ToClientMessage::CountdownUpdate(CountdownUpdateMessage {
          server_time_left_millis: time_left.as_millis(),
        });

        room
          .utils
          .broadcast_channel
          .send(msg)
          .map_err(BroadcastError::SendFailed)?;

        Ok(())
      })
      .await
  }

  pub fn apply_countdown(&self, room: &mut Room) {
    if let Some(countdown) = &room.utils.countdown {
      countdown.task.abort()
    }
    let app_state = self.clone();
    let room_id = room.utils.room_id;
    let now = Instant::now();
    match room.state {
      RoomState::RoundStart(_) => {
        // let stop = now + Duration::from_secs(180);
        let stop = now + Duration::from_secs(30);
        room.utils.countdown = Some(Countdown {
          task: tokio::spawn(async move {
            tokio::time::sleep_until(stop).await;
            app_state
              .apply_event(room_id, RoomEvent::FinalizeBids)
              .await
              .ok();
          }),
          stop,
        })
      }
      RoomState::RoundBidding(_) => {
        let stop = now + Duration::from_secs(60);
        room.utils.countdown = Some(Countdown {
          task: tokio::spawn(async move {
            tokio::time::sleep_until(stop).await;
            app_state
              .apply_event(room_id, RoomEvent::FinalizeBids)
              .await
              .ok();
          }),
          stop,
        })
      }
      RoomState::RoundSolving(_) => {
        let stop = now + Duration::from_secs(60);
        room.utils.countdown = Some(Countdown {
          task: tokio::spawn(async move {
            tokio::time::sleep_until(stop).await;
            app_state
              .apply_event(room_id, RoomEvent::YieldSolve)
              .await
              .ok();
          }),
          stop,
        })
      }
      _ => {}
    }
  }

  fn _apply_event(
    &self,
    room: &mut Room,
    event: RoomEvent,
  ) -> Result<(), ApplyEventError> {
    let original_discriminant = mem::discriminant(&room.state);
    let working_state = mem::replace(&mut room.state, RoomState::None);
    let event_is_yield_solve = matches!(event, RoomEvent::YieldSolve);

    let EventResult {
      result: next_state,
      error,
    } = working_state.apply(event);
    room.state = next_state;
    error.map(Err).unwrap_or(Ok(()))?;

    let current_discriminant = mem::discriminant(&room.state);
    let state_transition_occurred =
      original_discriminant != current_discriminant;
    let solver_changed =
      event_is_yield_solve && matches!(room.state, RoomState::RoundSolving(_));
    if state_transition_occurred || solver_changed {
      self.apply_countdown(room);
    }
    Ok(())
  }

  pub async fn apply_event_with_validation<F>(
    &self,
    room_id: RoomId,
    event: RoomEvent,
    predicate: F,
  ) -> Result<(), ApplyEventError>
  where
    F: FnOnce(&mut RoomState) -> bool,
  {
    self
      .with_room_write(room_id, |room| {
        if !predicate(&mut room.state) {
          return Err(ApplyEventError::ValidationFailedError);
        }
        self._apply_event(room, event)
      })
      .await
  }

  pub async fn apply_event(
    &self,
    room_id: RoomId,
    event: RoomEvent,
  ) -> Result<(), ApplyEventError> {
    self
      .with_room_write(room_id, |room| self._apply_event(room, event))
      .await
  }
}
