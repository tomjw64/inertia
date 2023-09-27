use inertia_core::board_generators::ClassicBoardGenerator;
use inertia_core::message::ToClientMessage;
use inertia_core::state::EventError;
use inertia_core::state::EventResult;
use inertia_core::state::PlayerId;
use inertia_core::state::RoomData;
use inertia_core::state::RoomEvent;
use inertia_core::state::RoomId;
use inertia_core::state::RoomMeta;
use inertia_core::state::RoomState;
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
  pub fn new(room_id: RoomId) -> Self {
    Room {
      utils: RoomUtils {
        channel: broadcast::channel(16).0,
      },
      state: Mutex::new(RoomState::RoundSummary {
        meta: RoomMeta {
          room_id,
          generator: Box::new(ClassicBoardGenerator::new()),
          players: HashMap::new(),
          player_reconnect_keys: HashMap::new(),
          player_last_seen: HashMap::new(),
          players_connected: HashMap::new(),
          player_scores: HashMap::new(),
          round_number: 0,
        },
      }),
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

// #[derive(Error, Debug)]
// pub enum RemovePlayerError {
//   #[error(transparent)]
//   NoRoomExists(#[from] NoRoomExistsError),
// }

// #[derive(Error, Debug)]
// pub enum RenamePlayerError {
//   #[error(transparent)]
//   NoRoomExists(#[from] NoRoomExistsError),
// }

// #[derive(Error, Debug)]
// pub enum StartNextRoundError {
//   #[error(transparent)]
//   NoRoomExists(#[from] NoRoomExistsError),
//   #[error(transparent)]
//   InvalidEvent(#[from] InvalidEventError),
// }

#[derive(Error, Debug)]
#[error("Room {} does not exist", self.0.0)]
pub struct NoRoomExistsError(RoomId);

// #[derive(Error, Debug)]
// #[error("Room {} is not in a valid state for event {}", self.0.0, self.1)]
// pub struct InvalidEventError(RoomId, String);

// impl InvalidEventError {
//   pub fn new<T: Into<String>>(room_id: RoomId, event: T) -> Self {
//     Self(room_id, event.into())
//   }
// }

fn broadcast_room(
) -> impl FnOnce(&RoomUtils, &mut RoomState) -> Result<(), BroadcastError> {
  |utils, state| {
    let msg = ToClientMessage::RoomUpdate(&*state);

    utils
      .channel
      .send(serde_json::to_string(&msg).unwrap())
      .map_err(BroadcastError::SendFailed)?;

    Ok(())
  }
}

// fn soft_remove_player(
//   player_id: PlayerId,
// ) -> impl FnOnce(&RoomMeta, &mut RoomData) -> Result<bool, RemovePlayerError> {
//   move |_, data| {
//     data.players_connected.remove(&player_id);
//     if matches!(data.state, RoomState::RoundSummary) && data.round_number == 0 {
//       data.players.remove(&player_id);
//       data.player_reconnect_keys.remove(&player_id);
//       data.player_scores.remove(&player_id);
//     }
//     Ok(data.players_connected.is_empty())
//   }
// }

// fn hard_remove_player(
//   player_id: PlayerId,
// ) -> impl FnOnce(&RoomMeta, &mut RoomData) -> Result<bool, RemovePlayerError> {
//   move |_, data| {
//     data.players_connected.remove(&player_id);
//     data.players.remove(&player_id);
//     data.player_reconnect_keys.remove(&player_id);
//     data.player_scores.remove(&player_id);

//     Ok(data.players_connected.is_empty())
//   }
// }

// fn start_next_round(
// ) -> impl FnOnce(&RoomMeta, &mut RoomData) -> Result<(), StartNextRoundError> {
//   move |meta, data| {
//     data.state = match data.state {
//       RoomState::RoundSummary => RoomState::RoundStart({
//         board: data.generator.generate_position(),
//       }),
//       _ => {
//         return Err(
//           InvalidEventError::new(data.room_id, "start_next_round").into(),
//         )
//       }
//     };
//     Ok(())
//   }
// }

// fn make_bid(
//   player_id: PlayerId,
//   player_bid: PlayerBid,
// ) -> impl FnOnce(&RoomMeta, &mut RoomData) -> Result<(), StartNextRoundError> {
//   move |meta, data| {
//     data.state = match &data.state {
//       RoomState::RoundStart(RoundStart { board }) => {
//         RoomState::RoundBidding(RoundBidding {
//           board,
//           player_bids: HashMap::new(),
//         })
//       }
//       RoomState::RoundBidding(RoundBidding { board, player_bids }) => {
//         RoomState::RoundBidding(RoundBidding { board, player_bids })
//       }
//       _ => return Err(InvalidEventError::new(data.room_id, "make_bid").into()),
//     };
//     Ok(())
//   }
// }

fn apply_event(
  event: RoomEvent,
) -> impl FnOnce(&RoomUtils, &mut RoomState) -> Result<(), ApplyEventError> {
  |_, state| {
    let working_state = mem::replace(state, RoomState::None);
    let EventResult {
      result: next_state,
      error,
    } = working_state.apply(event);
    *state = next_state;
    error.map(Err).unwrap_or(Ok(()))?;
    Ok(())
  }
}

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

  pub async fn broadcast_room(
    &self,
    room_id: RoomId,
  ) -> Result<(), BroadcastError> {
    self.with_room(room_id, broadcast_room()).await
  }

  pub async fn apply_event(
    &self,
    room_id: RoomId,
    event: RoomEvent,
  ) -> Result<(), ApplyEventError> {
    self.with_room(room_id, apply_event(event)).await
  }

  // pub async fn start_next_round(
  //   &self,
  //   room_id: RoomId,
  // ) -> Result<(), StartNextRoundError> {
  //   self.with_room(room_id, start_next_round()).await
  // }

  // pub async fn soft_remove_player(
  //   &self,
  //   room_id: RoomId,
  //   player_id: PlayerId,
  // ) -> Result<bool, RemovePlayerError> {
  //   self.with_room(room_id, soft_remove_player(player_id)).await
  // }

  // pub async fn hard_remove_player(
  //   &self,
  //   room_id: RoomId,
  //   player_id: PlayerId,
  // ) -> Result<bool, RemovePlayerError> {
  //   self.with_room(room_id, hard_remove_player(player_id)).await
  // }
}
