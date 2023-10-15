use thiserror::Error;

use crate::state::data::PlayerId;
use crate::state::data::PlayerInfo;
use crate::state::data::PlayerName;
use crate::state::data::PlayerReconnectKey;
use crate::state::data::RoomMeta;
use crate::state::data::RoomState;

use super::apply_event::RoomEvent;
use super::result::EventError;
use super::result::EventResult;

#[derive(Error, Debug)]
pub enum ConnectError {
  #[error("Invalid player name: {:?}", .0)]
  InvalidName(PlayerName),
  #[error("Player name already taken: {:?}", .0)]
  UsernameTaken(PlayerName),
  #[error("Bad reconnect key {:?} != {:?} for player {:?}", .0, .1, .2)]
  BadReconnectKey(PlayerReconnectKey, PlayerReconnectKey, PlayerName),
}

#[derive(Debug, Clone)]
pub struct Connect {
  pub player_name: PlayerName,
  pub player_id: PlayerId,
  pub player_reconnect_key: PlayerReconnectKey,
}

fn room_meta_connect(
  meta: &mut RoomMeta,
  event: Connect,
) -> Result<(), ConnectError> {
  let Connect {
    player_name,
    player_id,
    player_reconnect_key,
  } = event;
  if player_name.0.is_empty() {
    return Err(ConnectError::InvalidName(player_name));
  }

  let required_id = meta
    .player_info
    .iter()
    .find(|(_, name)| name.player_name == player_name)
    .map(|(id, _)| id);
  if let Some(&required_id) = required_id {
    if player_id != required_id {
      return Err(ConnectError::UsernameTaken(player_name));
    }
  }

  let required_reconnect_key = meta
    .player_info
    .get(&player_id)
    .map(|info| info.player_reconnect_key);
  if let Some(required_reconnect_key) = required_reconnect_key {
    if player_reconnect_key != required_reconnect_key {
      return Err(ConnectError::BadReconnectKey(
        player_reconnect_key,
        required_reconnect_key,
        player_name,
      ));
    }
  }

  meta
    .player_info
    .entry(player_id)
    .and_modify(|info| {
      info.player_connected = true;
      info.player_name = player_name.clone();
      info.player_last_seen = meta.round_number;
    })
    .or_insert_with(|| PlayerInfo {
      player_id,
      player_name: player_name.clone(),
      player_reconnect_key,
      player_last_seen: meta.round_number,
      player_connected: true,
      player_score: 0,
    });

  Ok(())
}

pub fn connect(mut state: RoomState, event: Connect) -> EventResult {
  if let Some(meta) = state.get_meta_mut() {
    if let Err(error) = room_meta_connect(meta, event) {
      EventResult::err(state, error)
    } else {
      EventResult::ok(state)
    }
  } else {
    EventResult {
      error: Some(EventError::IncompatibleState(
        state.to_string(),
        RoomEvent::Connect(event),
      )),
      result: state,
    }
  }
}
