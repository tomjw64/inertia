use crate::state::data::PlayerBid;
use crate::state::data::PlayerId;
use crate::state::data::RoomState;
use crate::state::data::RoundBidding;
use crate::state::data::RoundSolving;
use crate::state::data::RoundSummary;

use super::result::EventResult;

#[derive(Debug, Clone)]
pub struct ReadyBid {
  pub player_id: PlayerId,
}

#[derive(Debug, Clone)]
pub struct UnreadyBid {
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
    return EventResult::err(
      RoomState::RoundBidding(RoundBidding {
        player_bids,
        meta,
        board,
      }),
      error,
    );
  }

  if meta
    .player_info
    .iter()
    .map(|(some_player, _)| player_bids.get(*some_player))
    .all(|bid| matches!(bid, PlayerBid::ProspectiveReady { .. }))
  {
    let next_solver = player_bids.get_next_solver();
    return match next_solver {
      Some(next_solver_id) => {
        EventResult::ok(RoomState::RoundSolving(RoundSolving {
          meta,
          board,
          player_bids,
          solver: next_solver_id,
          solution: Vec::new(),
        }))
      }
      None => EventResult::ok(RoomState::RoundSummary(RoundSummary {
        meta,
        last_round_board: Some(board),
        last_round_solution: None,
        last_solver: None,
      })),
    };
  }

  EventResult::ok(RoomState::RoundBidding(RoundBidding {
    player_bids,
    meta,
    board,
  }))
}

pub fn round_bidding_unready_bid(
  state: RoundBidding,
  event: UnreadyBid,
) -> EventResult {
  let RoundBidding {
    meta,
    board,
    mut player_bids,
  } = state;
  let UnreadyBid { player_id } = event;

  if let Err(error) = player_bids.unready_bid(player_id) {
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
