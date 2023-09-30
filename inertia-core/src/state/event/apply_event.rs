use strum::Display;

use crate::state::data::RoomState;

use super::connect::connect;
use super::connect::Connect;
use super::disconnect::hard_disconnect;
use super::disconnect::soft_disconnect;
use super::disconnect::Disconnect;
use super::finalize_bids::round_bidding_finalize_bids;
use super::make_bid::round_bidding_make_bid;
use super::make_bid::round_start_make_bid;
use super::make_bid::MakeBid;
use super::result::EventError;
use super::result::EventResult;
use super::start_round::round_summary_start_round;
use super::update_solution::round_solving_update_solution;
use super::update_solution::UpdateSolution;
use super::yield_solve::round_solving_yield_solve;
use super::yield_solve::YieldSolve;

#[derive(Display, Debug)]
pub enum RoomEvent {
  Connect(Connect),
  SoftDisconnect(Disconnect),
  HardDisconnect(Disconnect),
  StartRound,
  MakeBid(MakeBid),
  FinalizeBids,
  UpdateSolution(UpdateSolution),
  YieldSolve(YieldSolve),
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
      (RoomState::RoundBidding(state), RoomEvent::MakeBid(event)) => {
        round_bidding_make_bid(state, event)
      }
      (RoomState::RoundBidding(state), RoomEvent::FinalizeBids) => {
        round_bidding_finalize_bids(state)
      }
      (RoomState::RoundSolving(state), RoomEvent::UpdateSolution(event)) => {
        round_solving_update_solution(state, event)
      }
      (RoomState::RoundSolving(state), RoomEvent::YieldSolve(event)) => {
        round_solving_yield_solve(state, event)
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
