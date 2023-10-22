use inertia_core::message::FromClientMessage;
use inertia_core::message::ToClientMessage;
use inertia_core::state::data::PlayerId;
use inertia_core::state::data::RoomId;
use inertia_core::state::event::apply_event::RoomEvent;
use inertia_core::state::event::lock_in_bid::LockInBid;
use inertia_core::state::event::make_bid::MakeBid;
use inertia_core::state::event::update_solution::UpdateSolution;
use thiserror::Error;
use tokio::sync::mpsc;
use tokio::sync::mpsc::error::SendError;

use crate::state::AppState;
use crate::state::ApplyEventError;
use crate::state::BroadcastError;

#[derive(Error, Debug)]
pub enum HandleMessageError {
  #[error("Unexpected message type: {}", .0)]
  UnexpectedMessageType(FromClientMessage),
  #[error(transparent)]
  ApplyEventError(#[from] ApplyEventError),
  #[error(transparent)]
  BroadcastError(#[from] BroadcastError),
  #[error("Failed to forward message to individual channel")]
  FailedToForwardMessage(#[from] SendError<ToClientMessage>),
}

pub async fn handle_message_from_client(
  msg: FromClientMessage,
  state: &AppState,
  room_id: RoomId,
  player_id: PlayerId,
  individual_sender: &mpsc::Sender<ToClientMessage>,
) -> Result<(), HandleMessageError> {
  match msg {
    FromClientMessage::ExplicitPing => {
      individual_sender
        .send(ToClientMessage::ExplicitPong)
        .await?;
    }
    FromClientMessage::Join(_) => {
      Err(HandleMessageError::UnexpectedMessageType(msg))?;
    }
    FromClientMessage::Rename(_) => {
      Err(HandleMessageError::UnexpectedMessageType(msg))?;
    }
    FromClientMessage::StartRound => {
      state.apply_event(room_id, RoomEvent::StartRound).await?;
    }
    FromClientMessage::Bid(bid_message) => {
      state
        .apply_event(
          room_id,
          RoomEvent::MakeBid(MakeBid {
            player_id,
            bid_value: bid_message.bid_value,
          }),
        )
        .await?
    }
    FromClientMessage::LockInBid => {
      state
        .apply_event(room_id, RoomEvent::LockInBid(LockInBid { player_id }))
        .await?
    }
    FromClientMessage::UpdateSolution(update_solution_message) => {
      state
        .apply_event_with_validation(
          room_id,
          RoomEvent::UpdateSolution(UpdateSolution {
            solution: update_solution_message.solution,
          }),
          |state| state.get_solver() == Some(player_id),
        )
        .await?
    }
    FromClientMessage::GiveUpSolve => {
      state
        .apply_event_with_validation(room_id, RoomEvent::YieldSolve, |state| {
          state.get_solver() == Some(player_id)
        })
        .await?
    }
  };
  Ok(())
}
