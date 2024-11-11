use std::collections::HashMap;
use std::hash::Hash;

use serde::Deserialize;
use serde::Serialize;
use strum::Display;
use thiserror::Error;

use crate::mechanics::Position;
use crate::mechanics::SolvedPositionGenerator;
use crate::solvers::Solution;

#[cfg(feature = "web")]
use {tsify::Tsify, wasm_bindgen::prelude::wasm_bindgen};

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone, Debug)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct PlayerId(pub u32);

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone, Debug)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct RoomId(pub u32);

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct PlayerName(pub String);

impl<T> From<T> for PlayerName
where
  T: Into<String>,
{
  fn from(value: T) -> Self {
    PlayerName(value.into())
  }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct PlayerReconnectKey(pub u32);

#[derive(Serialize, Deserialize, Eq, PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
#[serde(tag = "type", content = "content")]
pub enum PlayerBid {
  None,
  NoneReady,
  Prospective { value: u32, order: u32 },
  ProspectiveReady { value: u32, order: u32 },
  Failed { value: u32 },
}

impl PlayerBid {
  pub fn to_effective_value(self) -> u32 {
    match self {
      PlayerBid::None => 0,
      PlayerBid::NoneReady => 0,
      PlayerBid::Prospective { value, .. } => value,
      PlayerBid::ProspectiveReady { value, .. } => value,
      PlayerBid::Failed { value } => value,
    }
  }

  pub fn to_order(self) -> u32 {
    match self {
      PlayerBid::None => 0,
      PlayerBid::NoneReady => 0,
      PlayerBid::Prospective { order, .. } => order,
      PlayerBid::ProspectiveReady { order, .. } => order,
      PlayerBid::Failed { .. } => 0,
    }
  }

  pub fn to_ready(self) -> Self {
    match self {
      PlayerBid::None => PlayerBid::NoneReady,
      PlayerBid::Prospective { value, order } => {
        PlayerBid::ProspectiveReady { value, order }
      }
      _ => self,
    }
  }

  pub fn to_unready(self) -> Self {
    match self {
      PlayerBid::NoneReady => PlayerBid::None,
      PlayerBid::ProspectiveReady { value, order } => {
        PlayerBid::Prospective { value, order }
      }
      _ => self,
    }
  }

  pub fn to_failed(self) -> Self {
    let effective_value = self.to_effective_value();
    Self::Failed {
      value: effective_value,
    }
  }

  pub fn is_prospective(self) -> bool {
    matches!(
      self,
      PlayerBid::Prospective { .. } | PlayerBid::ProspectiveReady { .. }
    )
  }
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(into_wasm_abi))]
pub struct PlayerInfo {
  pub player_id: PlayerId,
  pub player_name: PlayerName,
  #[serde(skip)]
  pub player_reconnect_key: PlayerReconnectKey,
  #[serde(skip)]
  pub player_last_seen: u32,
  pub player_connected: bool,
  pub player_score: u32,
}

#[derive(Serialize, Debug, Clone)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(into_wasm_abi))]
pub struct RoomMeta {
  pub room_id: RoomId,
  #[serde(skip)]
  pub generator: Box<dyn SolvedPositionGenerator>,
  pub player_info: HashMap<PlayerId, PlayerInfo>,
  pub round_number: u32,
}

impl PartialEq for RoomMeta {
  fn eq(&self, other: &Self) -> bool {
    self.room_id == other.room_id
      && self.player_info == other.player_info
      && self.round_number == other.round_number
  }
}

impl Eq for RoomMeta {}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(into_wasm_abi))]
pub struct RoundSummary {
  pub meta: RoomMeta,
  pub last_round_board: Option<Position>,
  pub last_round_solution: Option<Solution>,
  pub last_solver: Option<PlayerId>,
  pub last_round_optimal_solution: Option<Solution>,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(into_wasm_abi))]
pub struct RoundStart {
  pub meta: RoomMeta,
  pub board: Position,
  #[serde(skip)]
  pub optimal_solution: Solution,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(into_wasm_abi))]
pub struct RoundBidding {
  pub meta: RoomMeta,
  pub board: Position,
  pub player_bids: PlayerBids,
  #[serde(skip)]
  pub optimal_solution: Solution,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(into_wasm_abi))]
pub struct RoundSolving {
  pub meta: RoomMeta,
  pub board: Position,
  pub player_bids: PlayerBids,
  pub solver: PlayerId,
  pub solution: Solution,
  #[serde(skip)]
  pub optimal_solution: Solution,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(into_wasm_abi, from_wasm_abi))]
pub struct PlayerBids {
  pub bids: HashMap<PlayerId, PlayerBid>,
  #[serde(skip)]
  pub timestamp: u32,
}

#[derive(Error, Debug)]
#[error("Unable to make a bid from the current state")]
pub struct MakeBidError;

#[derive(Error, Debug)]
#[error("Unable to ready bid from the current state")]
pub struct ReadyBidError;

#[derive(Error, Debug)]
#[error("Unable to unready bid from the current state")]
pub struct UnreadyBidError;

impl PlayerBids {
  pub fn get(&self, player_id: PlayerId) -> PlayerBid {
    *self.bids.get(&player_id).unwrap_or(&PlayerBid::None)
  }

  pub fn fail(&mut self, player_id: PlayerId) {
    let current_bid = self.get(player_id);
    self.bids.insert(player_id, current_bid.to_failed());
  }

  pub fn ready_bid(
    &mut self,
    player_id: PlayerId,
  ) -> Result<(), ReadyBidError> {
    let current_bid = self.bids.get(&player_id).unwrap_or(&PlayerBid::None);

    let can_update = matches!(
      current_bid,
      PlayerBid::None { .. } | PlayerBid::Prospective { .. }
    );

    if !can_update {
      return Err(ReadyBidError);
    }

    self.bids.insert(player_id, current_bid.to_ready());
    Ok(())
  }

  pub fn unready_bid(
    &mut self,
    player_id: PlayerId,
  ) -> Result<(), UnreadyBidError> {
    let current_bid = self.bids.get(&player_id).unwrap_or(&PlayerBid::None);

    let can_update = matches!(
      current_bid,
      PlayerBid::NoneReady { .. } | PlayerBid::ProspectiveReady { .. }
    );

    if !can_update {
      return Err(UnreadyBidError);
    }

    self.bids.insert(player_id, current_bid.to_unready());
    Ok(())
  }

  pub fn make_bid(
    &mut self,
    player_id: PlayerId,
    bid_value: u32,
  ) -> Result<(), MakeBidError> {
    let current_bid = self.bids.get(&player_id).unwrap_or(&PlayerBid::None);

    let can_update = match current_bid {
      PlayerBid::None => true,
      PlayerBid::Prospective { value, .. } if bid_value < *value => true,
      _ => false,
    };

    if !can_update {
      return Err(MakeBidError);
    }

    self.bids.insert(
      player_id,
      PlayerBid::Prospective {
        value: bid_value,
        order: self.timestamp,
      },
    );
    self.timestamp += 1;
    Ok(())
  }

  pub fn get_next_solver(&self) -> Option<PlayerId> {
    self
      .bids
      .iter()
      .filter(|(_, bid)| bid.is_prospective())
      .min_by_key(|(_, bid)| (bid.to_effective_value(), bid.to_order()))
      .map(|(id, _)| *id)
  }
}

#[derive(Serialize, Display, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "web", derive(Tsify), tsify(into_wasm_abi))]
#[serde(tag = "type", content = "content")]
pub enum RoomState {
  None,
  Closed,
  RoundSummary(RoundSummary),
  RoundStart(RoundStart),
  RoundBidding(RoundBidding),
  RoundSolving(RoundSolving),
}

impl RoomState {
  pub fn initial<T: SolvedPositionGenerator + 'static>(
    room_id: RoomId,
    generator: T,
  ) -> Self {
    RoomState::RoundSummary(RoundSummary {
      meta: RoomMeta {
        room_id,
        generator: Box::new(generator),
        player_info: HashMap::new(),
        round_number: 0,
      },
      last_round_board: None,
      last_round_solution: None,
      last_solver: None,
      last_round_optimal_solution: None,
    })
  }
  pub fn get_meta(&self) -> Option<&RoomMeta> {
    match self {
      RoomState::None => None,
      RoomState::Closed => None,
      RoomState::RoundSummary(RoundSummary { meta, .. }) => Some(meta),
      RoomState::RoundStart(RoundStart { meta, .. }) => Some(meta),
      RoomState::RoundBidding(RoundBidding { meta, .. }) => Some(meta),
      RoomState::RoundSolving(RoundSolving { meta, .. }) => Some(meta),
    }
  }
  pub fn get_meta_mut(&mut self) -> Option<&mut RoomMeta> {
    match self {
      RoomState::None => None,
      RoomState::Closed => None,
      RoomState::RoundSummary(RoundSummary { meta, .. }) => Some(meta),
      RoomState::RoundStart(RoundStart { meta, .. }) => Some(meta),
      RoomState::RoundBidding(RoundBidding { meta, .. }) => Some(meta),
      RoomState::RoundSolving(RoundSolving { meta, .. }) => Some(meta),
    }
  }
  pub fn get_solver(&self) -> Option<PlayerId> {
    match self {
      RoomState::None => None,
      RoomState::Closed => None,
      RoomState::RoundSummary(_) => None,
      RoomState::RoundStart(_) => None,
      RoomState::RoundBidding(_) => None,
      RoomState::RoundSolving(RoundSolving { solver, .. }) => Some(*solver),
    }
  }
}
