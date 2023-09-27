use crate::state::data::RoomState;

use super::connect::round_summary_connect;
use super::connect::Connect;
use super::disconnect::round_summary_soft_disconnect;
use super::disconnect::SoftDisconnect;
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

#[derive(Debug)]
pub enum RoomEvent {
  Connect(Connect),
  SoftDisconnect(SoftDisconnect),
  StartRound,
  MakeBid(MakeBid),
  FinalizeBids,
  UpdateSolution(UpdateSolution),
  YieldSolve(YieldSolve),
}

impl RoomState {
  pub fn apply(self, event: RoomEvent) -> EventResult {
    match (self, event) {
      (RoomState::RoundSummary(state), RoomEvent::Connect(event)) => {
        round_summary_connect(state, event)
      }
      (RoomState::RoundSummary(state), RoomEvent::SoftDisconnect(event)) => {
        round_summary_soft_disconnect(state, event)
      }
      (RoomState::RoundSummary(state), RoomEvent::StartRound) => {
        round_summary_start_round(state)
      }
      (RoomState::RoundStart(state), RoomEvent::MakeBid(event)) => {
        round_start_make_bid(state, event)
      }
      (RoomState::RoundBidding(state), RoomEvent::MakeBid(event)) => {
        round_bidding_make_bid(state, event)
      }
      (RoomState::RoundSolving(state), RoomEvent::UpdateSolution(event)) => {
        round_solving_update_solution(state, event)
      }
      (RoomState::RoundSolving(state), RoomEvent::YieldSolve(event)) => {
        round_solving_yield_solve(state, event)
      }
      (state, event) => EventResult {
        error: Some(EventError::IncompatibleState(state.to_string(), event)),
        result: state,
      },
    }
  }
}
