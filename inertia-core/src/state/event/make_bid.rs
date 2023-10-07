use thiserror::Error;

use crate::state::data::PlayerBids;
use crate::state::data::PlayerId;
use crate::state::data::RoomState;
use crate::state::data::RoundBidding;
use crate::state::data::RoundStart;

use super::result::EventResult;

#[derive(Debug)]
pub struct MakeBid {
  pub player_id: PlayerId,
  pub bid_value: usize,
}

pub fn round_start_make_bid(state: RoundStart, event: MakeBid) -> EventResult {
  let RoundStart { meta, board } = state;
  let MakeBid {
    player_id,
    bid_value,
  } = event;
  let mut player_bids = PlayerBids::default();
  player_bids.make_bid(player_id, bid_value);
  EventResult::ok(RoomState::RoundBidding(RoundBidding {
    player_bids,
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
    bid_value,
  } = event;
  player_bids.make_bid(player_id, bid_value);
  EventResult::ok(RoomState::RoundBidding(RoundBidding {
    meta,
    board,
    player_bids,
  }))
}
