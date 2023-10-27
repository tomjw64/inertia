use crate::state::data::PlayerId;
use crate::state::data::RoomState;
use crate::state::data::RoundBidding;

use super::result::EventResult;

#[derive(Debug, Clone)]
pub struct ReadyBid {
  pub player_id: PlayerId,
}

pub fn round_bidding_ready_bid(
  state: RoundBidding,
  event: ReadyBid,
) -> EventResult {
  let RoundBidding {
    meta,
    board,
    mut player_bids,
  } = state;
  let ReadyBid { player_id } = event;

  if let Err(error) = player_bids.ready_bid(player_id) {
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
