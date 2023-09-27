use std::collections::HashMap;

use crate::state::data::PlayerBid;
use crate::state::data::PlayerId;
use crate::state::data::RoomState;
use crate::state::data::RoundBidding;
use crate::state::data::RoundStart;

use super::result::EventResult;

#[derive(Debug)]
pub struct MakeBid {
  pub player_id: PlayerId,
  pub player_bid: PlayerBid,
}

pub fn round_start_make_bid(state: RoundStart, event: MakeBid) -> EventResult {
  let RoundStart { meta, board } = state;
  let MakeBid {
    player_id,
    player_bid,
  } = event;
  EventResult::ok(RoomState::RoundBidding(RoundBidding {
    player_bids: HashMap::from([(player_id, player_bid)]),
    meta,
    board,
  }))
}

pub fn round_bidding_make_bid(
  state: RoundBidding,
  event: MakeBid,
) -> EventResult {
  let RoundBidding {
    meta,
    board,
    mut player_bids,
  } = state;
  let MakeBid {
    player_id,
    player_bid,
  } = event;
  player_bids.insert(player_id, player_bid);
  EventResult::ok(RoomState::RoundBidding(RoundBidding {
    meta,
    board,
    player_bids,
  }))
}
