use strum::Display;

use crate::state::data::RoomState;

use super::connect::connect;
use super::connect::Connect;
use super::disconnect::hard_disconnect;
use super::disconnect::soft_disconnect;
use super::disconnect::Disconnect;
use super::finalize_bids::round_bidding_finalize_bids;
use super::finalize_bids::round_start_finalize_bids;
use super::make_bid::round_bidding_make_bid;
use super::make_bid::round_start_make_bid;
use super::make_bid::MakeBid;
use super::ready_bid::round_bidding_ready_bid;
use super::ready_bid::round_bidding_unready_bid;
use super::ready_bid::ReadyBid;
use super::ready_bid::UnreadyBid;
use super::result::EventError;
use super::result::EventResult;
use super::start_round::round_summary_start_round;
use super::update_solution::round_solving_update_solution;
use super::update_solution::UpdateSolution;
use super::yield_solve::round_solving_yield_solve;

#[derive(Display, Debug, Clone)]
pub enum RoomEvent {
  Connect(Connect),
  SoftDisconnect(Disconnect),
  HardDisconnect(Disconnect),
  StartRound,
  MakeBid(MakeBid),
  ReadyBid(ReadyBid),
  UnreadyBid(UnreadyBid),
  FinalizeBids,
  UpdateSolution(UpdateSolution),
  YieldSolve,
}

impl RoomState {
  pub fn apply(self, event: RoomEvent) -> EventResult {
    match (self, event) {
      (RoomState::RoundSummary(state), RoomEvent::StartRound) => {
        round_summary_start_round(state)
      }
      (RoomState::RoundStart(state), RoomEvent::MakeBid(event)) => {
        round_start_make_bid(state, event)
      }
      (RoomState::RoundStart(state), RoomEvent::FinalizeBids) => {
        round_start_finalize_bids(state)
      }
      (RoomState::RoundBidding(state), RoomEvent::MakeBid(event)) => {
        round_bidding_make_bid(state, event)
      }
      (RoomState::RoundBidding(state), RoomEvent::FinalizeBids) => {
        round_bidding_finalize_bids(state)
      }
      (RoomState::RoundBidding(state), RoomEvent::ReadyBid(event)) => {
        round_bidding_ready_bid(state, event)
      }
      (RoomState::RoundBidding(state), RoomEvent::UnreadyBid(event)) => {
        round_bidding_unready_bid(state, event)
      }
      (RoomState::RoundSolving(state), RoomEvent::UpdateSolution(event)) => {
        round_solving_update_solution(state, event)
      }
      (RoomState::RoundSolving(state), RoomEvent::YieldSolve) => {
        round_solving_yield_solve(state)
      }
      (state, RoomEvent::HardDisconnect(event)) => {
        hard_disconnect(state, event)
      }
      (state, RoomEvent::SoftDisconnect(event)) => {
        soft_disconnect(state, event)
      }
      (state, RoomEvent::Connect(event)) => connect(state, event),
      (state, event) => EventResult {
        error: Some(EventError::IncompatibleState(state.to_string(), event)),
        result: state,
      },
    }
  }
}
