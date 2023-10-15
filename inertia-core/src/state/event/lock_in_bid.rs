use crate::state::data::PlayerId;
use crate::state::data::RoomState;
use crate::state::data::RoundBidding;

use super::result::EventResult;

#[derive(Debug, Clone)]
pub struct LockInBid {
  pub player_id: PlayerId,
}

pub fn round_bidding_lock_in_bid(
  state: RoundBidding,
  event: LockInBid,
) -> EventResult {
  let RoundBidding {
    meta,
    board,
    mut player_bids,
  } = state;
  let LockInBid { player_id } = event;

  if let Err(error) = player_bids.lock_in_bid(player_id) {
    EventResult::err(
      RoomState::RoundBidding(RoundBidding {
        player_bids,
        meta,
        board,
      }),
      error,
    )
  } else {
    EventResult::ok(RoomState::RoundBidding(RoundBidding {
      player_bids,
      meta,
      board,
    }))
  }
}
