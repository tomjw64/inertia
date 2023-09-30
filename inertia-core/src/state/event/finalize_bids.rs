use crate::state::data::RoomState;
use crate::state::data::RoundBidding;
use crate::state::data::RoundSolving;
use crate::state::data::RoundSummary;

use super::result::EventResult;

pub fn round_bidding_finalize_bids(state: RoundBidding) -> EventResult {
  let RoundBidding {
    meta,
    board,
    player_bids,
  } = state;

  let next_bidder = player_bids
    .iter()
    .filter(|(_, bid)| bid.is_prospective())
    .min_by_key(|(_, bid)| bid.to_effective_value())
    .map(|(id, _)| id);

  match next_bidder {
    Some(&next_bidder_id) => {
      EventResult::ok(RoomState::RoundSolving(RoundSolving {
        meta,
        board,
        player_bids,
        solver: next_bidder_id,
        solution: Vec::new(),
      }))
    }
    None => EventResult::ok(RoomState::RoundSummary(RoundSummary {
      meta,
      last_round_board: Some(board),
      last_round_solution: None,
    })),
  }
}
