use std::collections::HashMap;
use std::hash::Hash;

use serde::Deserialize;
use serde::Serialize;
use strum::Display;
use typeshare::typeshare;

use crate::mechanics::WalledBoardPosition;
use crate::mechanics::WalledBoardPositionGenerator;
use crate::solvers::SolutionStep;

#[typeshare(serialized_as = "number")]
#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct PlayerId(pub usize);

#[typeshare(serialized_as = "number")]
#[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct RoomId(pub usize);

#[typeshare]
#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct PlayerName(pub String);

#[typeshare(serialized_as = "number")]
#[derive(Serialize, Deserialize, Eq, PartialEq, Copy, Clone, Debug)]
pub struct PlayerReconnectKey(pub usize);

#[typeshare]
#[derive(Serialize, Deserialize, Eq, PartialEq, Copy, Clone, Debug)]
pub enum PlayerBid {
  None,
  Prospective {
    #[typeshare(serialized_as = "number")]
    value: usize,
  },
  ProspectiveLocked {
    #[typeshare(serialized_as = "number")]
    value: usize,
  },
  Failed {
    #[typeshare(serialized_as = "number")]
    value: usize,
  },
}

impl PlayerBid {
  pub fn to_effective_value(self) -> usize {
    match self {
      PlayerBid::None => 0,
      PlayerBid::Prospective { value } => value,
      PlayerBid::ProspectiveLocked { value } => value,
      PlayerBid::Failed { value } => value,
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
      PlayerBid::Prospective { .. } | PlayerBid::ProspectiveLocked { .. }
    )
  }
}

#[typeshare]
#[derive(Serialize, Debug)]
pub struct PlayerInfo {
  pub player_id: PlayerId,
  pub player_name: PlayerName,
  #[serde(skip)]
  pub player_reconnect_key: PlayerReconnectKey,
  #[serde(skip)]
  pub player_last_seen: usize,
  pub player_connected: bool,
  #[typeshare(typescript(type = "number"))]
  pub player_score: usize,
}

#[typeshare]
#[derive(Serialize, Debug)]
pub struct RoomMeta {
  pub room_id: RoomId,
  #[serde(skip)]
  pub generator: Box<dyn WalledBoardPositionGenerator>,
  pub player_info: HashMap<PlayerId, PlayerInfo>,
  #[typeshare(typescript(type = "number"))]
  pub round_number: usize,
}

#[typeshare]
#[derive(Serialize, Debug)]
pub struct RoundSummary {
  pub meta: RoomMeta,
  pub last_round_board: Option<WalledBoardPosition>,
  pub last_round_solution: Option<Vec<SolutionStep>>,
}

#[typeshare]
#[derive(Serialize, Debug)]
pub struct RoundStart {
  pub meta: RoomMeta,
  pub board: WalledBoardPosition,
}

#[typeshare]
#[derive(Serialize, Debug)]
pub struct RoundBidding {
  pub meta: RoomMeta,
  pub board: WalledBoardPosition,
  pub player_bids: HashMap<PlayerId, PlayerBid>,
}

#[typeshare]
#[derive(Serialize, Debug)]
pub struct RoundSolving {
  pub meta: RoomMeta,
  pub board: WalledBoardPosition,
  pub player_bids: HashMap<PlayerId, PlayerBid>,
  pub solver: PlayerId,
  pub solution: Vec<SolutionStep>,
}

#[typeshare]
#[derive(Serialize, Display, Debug)]
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
  pub fn initial<T: WalledBoardPositionGenerator + 'static>(
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
}
