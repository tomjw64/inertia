use crate::state::data::PlayerBids;
use crate::state::data::PlayerId;
use crate::state::data::RoomState;
use crate::state::data::RoundBidding;
use crate::state::data::RoundStart;

use super::result::EventResult;

#[derive(Debug, Clone)]
pub struct MakeBid {
  pub player_id: PlayerId,
  pub bid_value: u32,
}

pub fn round_start_make_bid(state: RoundStart, event: MakeBid) -> EventResult {
  let RoundStart {
    meta,
    board,
    optimal_solution,
  } = state;
  let MakeBid {
    player_id,
    bid_value,
  } = event;
  let mut player_bids = PlayerBids::default();
  if let Err(error) = player_bids.make_bid(player_id, bid_value) {
    EventResult::err(
      RoomState::RoundBidding(RoundBidding {
        player_bids,
        meta,
        board,
        optimal_solution,
      }),
      error,
    )
  } else {
    EventResult::ok(RoomState::RoundBidding(RoundBidding {
      player_bids,
      meta,
      board,
      optimal_solution,
    }))
  }
}

pub fn round_bidding_make_bid(
  state: RoundBidding,
  event: MakeBid,
) -> EventResult {
  let RoundBidding {
    meta,
    board,
    optimal_solution,
    mut player_bids,
  } = state;
  let MakeBid {
    player_id,
    bid_value,
  } = event;
  if let Err(error) = player_bids.make_bid(player_id, bid_value) {
    EventResult::err(
      RoomState::RoundBidding(RoundBidding {
        player_bids,
        meta,
        board,
        optimal_solution,
      }),
      error,
    )
  } else {
    EventResult::ok(RoomState::RoundBidding(RoundBidding {
      player_bids,
      meta,
      board,
      optimal_solution,
    }))
  }
}
