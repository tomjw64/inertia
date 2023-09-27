pub mod data;
pub mod event;

// use std::collections::HashMap;
// use std::hash::Hash;

// use serde::Deserialize;
// use serde::Serialize;
// use strum::Display;
// use thiserror::Error;
// use typeshare::typeshare;

// use crate::mechanics::WalledBoardPosition;
// use crate::mechanics::WalledBoardPositionGenerator;
// use crate::solvers::SolutionStep;

// #[typeshare(serialized_as = "number")]
// #[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone, Debug)]
// pub struct PlayerId(pub usize);

// #[typeshare(serialized_as = "number")]
// #[derive(Serialize, Deserialize, Eq, PartialEq, Hash, Copy, Clone, Debug)]
// pub struct RoomId(pub usize);

// #[typeshare]
// #[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
// pub struct PlayerName(pub String);

// // #[typeshare(serialized_as = "number")]
// // #[derive(Serialize, Deserialize, Eq, PartialEq, Copy, Clone, Debug)]
// // pub struct PlayerScore(pub usize);

// #[typeshare(serialized_as = "number")]
// #[derive(Serialize, Deserialize, Eq, PartialEq, Copy, Clone, Debug)]
// pub struct PlayerReconnectKey(pub usize);

// #[typeshare(serialized_as = "number")]
// #[derive(Serialize, Deserialize, Eq, PartialEq, Copy, Clone, Debug)]
// pub struct PlayerBid(usize);

// // #[typeshare]
// // #[derive(Serialize)]
// // pub struct RoomData {
// //   pub room_id: RoomId,
// //   #[serde(skip)]
// //   pub generator: Box<dyn WalledBoardPositionGenerator>,
// //   #[serde(skip)]
// //   pub player_reconnect_keys: HashMap<PlayerId, PlayerReconnectKey>,
// //   #[serde(skip)]
// //   pub players: HashMap<PlayerId, PlayerName>,
// //   pub player_last_seen: HashMap<PlayerId, usize>,
// //   pub players_connected: HashMap<PlayerId, bool>,
// //   pub player_scores: HashMap<PlayerId, PlayerScore>,
// //   #[typeshare(typescript(type = "number"))]
// //   pub round_number: usize,
// //   pub state: RoomState,
// // }

// pub trait DebuggableGenerator:
//   WalledBoardPositionGenerator + std::fmt::Debug
// {
// }

// #[typeshare]
// #[derive(Serialize, Debug)]
// pub struct PlayerInfo {
//   player_id: PlayerId,
//   player_name: PlayerName,
//   #[serde(skip)]
//   player_reconnect_key: PlayerReconnectKey,
//   #[serde(skip)]
//   player_last_seen: usize,
//   player_connected: bool,
//   #[typeshare(typescript(type = "number"))]
//   player_score: usize,
// }

// #[typeshare]
// #[derive(Serialize, Debug)]
// pub struct RoomMeta {
//   pub room_id: RoomId,
//   #[serde(skip)]
//   pub generator: Box<dyn DebuggableGenerator>,
//   pub player_info: HashMap<PlayerId, PlayerInfo>,
//   // #[serde(skip)]
//   // pub player_reconnect_keys: HashMap<PlayerId, PlayerReconnectKey>,
//   // #[serde(skip)]
//   // pub player_last_seen: HashMap<PlayerId, usize>,
//   // pub players: HashMap<PlayerId, PlayerName>,
//   // pub players_connected: HashMap<PlayerId, bool>,
//   // pub player_scores: HashMap<PlayerId, PlayerScore>,
//   #[typeshare(typescript(type = "number"))]
//   pub round_number: usize,
// }

// #[typeshare]
// #[derive(Serialize, Debug)]
// pub struct RoundSummary {
//   meta: RoomMeta,
// }

// #[typeshare]
// #[derive(Serialize, Debug)]
// pub struct RoundStart {
//   meta: RoomMeta,
//   board: WalledBoardPosition,
// }

// #[typeshare]
// #[derive(Serialize, Debug)]
// pub struct RoundBidding {
//   meta: RoomMeta,
//   board: WalledBoardPosition,
//   player_bids: HashMap<PlayerId, PlayerBid>,
// }

// #[typeshare]
// #[derive(Serialize, Debug)]
// pub struct RoundSolving {
//   meta: RoomMeta,
//   board: WalledBoardPosition,
//   player_bids: HashMap<PlayerId, PlayerBid>,
//   solver: PlayerId,
//   solution: Vec<SolutionStep>,
// }

// #[typeshare]
// #[derive(Serialize, Display, Debug)]
// #[serde(tag = "type", content = "content")]
// pub enum RoomState {
//   None,
//   Closed,
//   RoundSummary(RoundSummary),
//   RoundStart(RoundStart),
//   RoundBidding(RoundBidding),
//   RoundSolving(RoundSolving),
// }

// #[derive(Debug)]
// pub struct Connect {
//   player_name: PlayerName,
//   player_id: PlayerId,
//   player_reconnect_key: PlayerReconnectKey,
// }

// #[derive(Debug)]
// pub struct SoftDisconnect {
//   player_id: PlayerId,
// }

// #[derive(Debug)]
// pub struct MakeBid {
//   player_id: PlayerId,
//   player_bid: PlayerBid,
// }

// #[derive(Debug)]
// pub enum RoomEvent {
//   Connect(Connect),
//   SoftDisconnect(SoftDisconnect),
//   StartRound,
//   MakeBid(MakeBid),
// }

// #[derive(Error, Debug)]
// pub enum ConnectError {
//   #[error("Invalid player name: {:?}", .0)]
//   InvalidName(PlayerName),
//   #[error("Player name already taken: {:?}", .0)]
//   UsernameTaken(PlayerName),
//   #[error("Bad reconnect key {:?} != {:?} for player {:?}", .0, .1, .2)]
//   BadReconnectKey(PlayerReconnectKey, PlayerReconnectKey, PlayerName),
// }

// #[derive(Error, Debug)]
// pub enum EventError {
//   #[error("State {} is incompatible with event {:?}", .0, .1)]
//   IncompatibleState(String, RoomEvent),
//   #[error(transparent)]
//   ConnectError(#[from] ConnectError),
// }

// pub struct EventResult {
//   pub result: RoomState,
//   pub error: Option<EventError>,
// }

// impl EventResult {
//   pub fn ok(result: RoomState) -> Self {
//     EventResult {
//       result,
//       error: None,
//     }
//   }
//   pub fn err<E: Into<EventError>>(result: RoomState, error: E) -> Self {
//     EventResult {
//       result,
//       error: Some(error.into()),
//     }
//   }
// }

// fn round_summary_round_start(state: RoundSummary) -> EventResult {
//   let RoundSummary { meta } = state;
//   EventResult::ok(RoomState::RoundStart(RoundStart {
//     board: meta.generator.generate_position(),
//     meta,
//   }))
// }

// fn round_start_make_bid(state: RoundStart, event: MakeBid) -> EventResult {
//   let RoundStart { meta, board } = state;
//   let MakeBid {
//     player_id,
//     player_bid,
//   } = event;
//   EventResult::ok(RoomState::RoundBidding(RoundBidding {
//     player_bids: HashMap::from([(player_id, player_bid)]),
//     meta,
//     board,
//   }))
// }

// fn round_bidding_make_bid(state: RoundBidding, event: MakeBid) -> EventResult {
//   let RoundBidding {
//     meta,
//     board,
//     mut player_bids,
//   } = state;
//   let MakeBid {
//     player_id,
//     player_bid,
//   } = event;
//   player_bids.insert(player_id, player_bid);
//   EventResult::ok(RoomState::RoundBidding(RoundBidding {
//     meta,
//     board,
//     player_bids,
//   }))
// }

// fn room_meta_connect(
//   meta: &mut RoomMeta,
//   event: Connect,
// ) -> Result<(), ConnectError> {
//   let Connect {
//     player_name,
//     player_id,
//     player_reconnect_key,
//   } = event;
//   if player_name.0.is_empty() {
//     return Err(ConnectError::InvalidName(player_name));
//   }

//   let required_id = meta
//     .player_info
//     .iter()
//     .find(|(_, name)| name.player_name == player_name)
//     .map(|(id, _)| id);
//   if let Some(&required_id) = required_id {
//     if player_id != required_id {
//       return Err(ConnectError::UsernameTaken(player_name));
//     }
//   }

//   let required_reconnect_key = meta
//     .player_info
//     .get(&player_id)
//     .map(|info| info.player_reconnect_key);
//   if let Some(required_reconnect_key) = required_reconnect_key {
//     if player_reconnect_key != required_reconnect_key {
//       return Err(ConnectError::BadReconnectKey(
//         player_reconnect_key,
//         required_reconnect_key,
//         player_name,
//       ));
//     }
//   }

//   meta
//     .player_info
//     .entry(player_id)
//     .and_modify(|info| {
//       info.player_connected = true;
//       info.player_name = player_name.clone();
//       info.player_last_seen = meta.round_number;
//     })
//     .or_insert_with(|| PlayerInfo {
//       player_id: player_id,
//       player_name: player_name.clone(),
//       player_reconnect_key: player_reconnect_key,
//       player_last_seen: meta.round_number,
//       player_connected: true,
//       player_score: 0,
//     });

//   Ok(())
// }

// fn round_summary_connect(
//   mut state: RoundSummary,
//   event: Connect,
// ) -> EventResult {
//   if let Err(error) = room_meta_connect(&mut state.meta, event) {
//     EventResult::err(RoomState::RoundSummary(state), error)
//   } else {
//     EventResult::ok(RoomState::RoundSummary(state))
//   }
// }

// fn room_meta_hard_disconnect(meta: &mut RoomMeta, event: SoftDisconnect) {
//   let SoftDisconnect { player_id } = event;
//   meta.player_info.remove(&player_id);
// }

// fn room_meta_soft_disconnect(meta: &mut RoomMeta, event: SoftDisconnect) {
//   let SoftDisconnect { player_id } = event;
//   meta
//     .player_info
//     .entry(player_id)
//     .and_modify(|e| e.player_connected = false);
// }

// fn round_summary_soft_disconnect(
//   mut state: RoundSummary,
//   event: SoftDisconnect,
// ) -> EventResult {
//   room_meta_soft_disconnect(&mut state.meta, event);
//   let players_still_present = state
//     .meta
//     .player_info
//     .iter()
//     .any(|(_, info)| info.player_connected);
//   if players_still_present {
//     EventResult::ok(RoomState::RoundSummary(state))
//   } else {
//     EventResult::ok(RoomState::Closed)
//   }
// }

// impl RoomState {
//   pub fn apply(self, event: RoomEvent) -> EventResult {
//     match (self, event) {
//       (RoomState::RoundSummary(state), RoomEvent::Connect(event)) => {
//         round_summary_connect(state, event)
//       }
//       (RoomState::RoundSummary(state), RoomEvent::SoftDisconnect(event)) => {
//         round_summary_soft_disconnect(state, event)
//       }
//       (RoomState::RoundSummary(state), RoomEvent::StartRound) => {
//         round_summary_round_start(state)
//       }
//       (RoomState::RoundStart(state), RoomEvent::MakeBid(event)) => {
//         round_start_make_bid(state, event)
//       }
//       (RoomState::RoundBidding(state), RoomEvent::MakeBid(event)) => {
//         round_bidding_make_bid(state, event)
//       }
//       (state, event) => EventResult {
//         error: Some(EventError::IncompatibleState(state.to_string(), event)),
//         result: state,
//       },
//     }
//   }
// }

// // #[typeshare]
// // #[derive(Serialize)]
// // #[serde(tag = "type", content = "content")]
// // pub enum RoomState {
// //   RoundSummary,
// //   RoundStart {
// //     board: WalledBoardPosition,
// //   },
// //   RoundBidding {
// //     board: WalledBoardPosition,
// //     player_bids: HashMap<PlayerId, PlayerBid>,
// //   },
// //   RoundSolving {
// //     board: WalledBoardPosition,
// //     player_bids: HashMap<PlayerId, PlayerBid>,
// //     solver: PlayerId,
// //     solution: Vec<SolutionStep>,
// //   },
// // }
