pub mod data;
pub mod event;

#[cfg(test)]
mod test {
  use pretty_assertions::assert_eq;
  use std::collections::HashMap;

  use crate::board_generators::OneMoveSolutionBoardGenerator;
  use crate::mechanics::Direction;
  use crate::mechanics::WalledBoardPosition;
  use crate::mechanics::WalledBoardPositionGenerator;
  use crate::solvers::SolutionStep;
  use crate::state::data::PlayerBids;
  use crate::state::data::PlayerId;
  use crate::state::data::PlayerInfo;
  use crate::state::data::PlayerName;
  use crate::state::data::PlayerReconnectKey;
  use crate::state::data::RoomId;
  use crate::state::data::RoomMeta;
  use crate::state::data::RoomState;
  use crate::state::data::RoundSolving;
  use crate::state::data::RoundStart;
  use crate::state::data::RoundSummary;
  use crate::state::event::apply_event::RoomEvent;
  use crate::state::event::connect::Connect;
  use crate::state::event::disconnect::Disconnect;

  use super::data::PlayerBid;
  use super::event::make_bid::MakeBid;
  use super::event::update_solution::UpdateSolution;

  fn expected_board() -> WalledBoardPosition {
    OneMoveSolutionBoardGenerator::new().generate_position()
  }

  fn expected_generator() -> Box<dyn WalledBoardPositionGenerator> {
    Box::new(OneMoveSolutionBoardGenerator::new())
  }

  fn expected_room() -> RoomId {
    RoomId(0)
  }

  fn simulate(events: Vec<RoomEvent>) -> RoomState {
    let state =
      RoomState::initial(RoomId(0), OneMoveSolutionBoardGenerator::new());
    simulate_on(state, events)
  }

  fn simulate_on(mut state: RoomState, events: Vec<RoomEvent>) -> RoomState {
    for event in events {
      state = state.apply(event).result;
      assert!(!matches!(state, RoomState::None));
    }
    state
  }

  #[test]
  fn initial_state() {
    let events = vec![];
    let result = simulate(events);
    assert_eq!(
      result,
      RoomState::RoundSummary(RoundSummary {
        meta: RoomMeta {
          room_id: expected_room(),
          generator: expected_generator(),
          player_info: HashMap::new(),
          round_number: 0
        },
        last_round_board: None,
        last_round_solution: None,
        last_solver: None
      })
    );
  }

  #[test]
  fn connect() {
    let events = vec![RoomEvent::Connect(Connect {
      player_name: PlayerName::from("test"),
      player_id: PlayerId(1),
      player_reconnect_key: PlayerReconnectKey(123),
    })];
    let result = simulate(events);
    assert_eq!(
      result,
      RoomState::RoundSummary(RoundSummary {
        meta: RoomMeta {
          room_id: expected_room(),
          generator: expected_generator(),
          player_info: HashMap::from([(
            PlayerId(1),
            PlayerInfo {
              player_id: PlayerId(1),
              player_name: PlayerName::from("test"),
              player_reconnect_key: PlayerReconnectKey(123),
              player_last_seen: 0,
              player_connected: true,
              player_score: 0
            }
          )]),
          round_number: 0
        },
        last_round_board: None,
        last_round_solution: None,
        last_solver: None
      })
    );
  }

  #[test]
  fn bad_connect() {
    let events = vec![
      RoomEvent::Connect(Connect {
        player_name: PlayerName::from("test"),
        player_id: PlayerId(1),
        player_reconnect_key: PlayerReconnectKey(123),
      }),
      // Duplicate username, different ID
      RoomEvent::Connect(Connect {
        player_name: PlayerName::from("test"),
        player_id: PlayerId(2),
        player_reconnect_key: PlayerReconnectKey(123),
      }),
      // Same username and player ID, different reconnect key
      RoomEvent::Connect(Connect {
        player_name: PlayerName::from("test"),
        player_id: PlayerId(1),
        player_reconnect_key: PlayerReconnectKey(321),
      }),
    ];
    let result = simulate(events);
    assert_eq!(
      result,
      RoomState::RoundSummary(RoundSummary {
        meta: RoomMeta {
          room_id: expected_room(),
          generator: expected_generator(),
          player_info: HashMap::from([(
            PlayerId(1),
            PlayerInfo {
              player_id: PlayerId(1),
              player_name: PlayerName::from("test"),
              player_reconnect_key: PlayerReconnectKey(123),
              player_last_seen: 0,
              player_connected: true,
              player_score: 0
            }
          )]),
          round_number: 0
        },
        last_round_board: None,
        last_round_solution: None,
        last_solver: None
      })
    );
  }

  #[test]
  fn disconnect() {
    let events = vec![
      RoomEvent::Connect(Connect {
        player_name: PlayerName::from("test"),
        player_id: PlayerId(1),
        player_reconnect_key: PlayerReconnectKey(123),
      }),
      RoomEvent::Connect(Connect {
        player_name: PlayerName::from("test2"),
        player_id: PlayerId(2),
        player_reconnect_key: PlayerReconnectKey(321),
      }),
      RoomEvent::SoftDisconnect(Disconnect {
        player_id: PlayerId(2),
      }),
      RoomEvent::StartRound,
    ];
    let result = simulate(events);
    assert_eq!(
      result,
      RoomState::RoundStart(RoundStart {
        meta: RoomMeta {
          room_id: expected_room(),
          generator: expected_generator(),
          player_info: HashMap::from([
            (
              PlayerId(1),
              PlayerInfo {
                player_id: PlayerId(1),
                player_name: PlayerName::from("test"),
                player_reconnect_key: PlayerReconnectKey(123),
                player_last_seen: 1,
                player_connected: true,
                player_score: 0
              }
            ),
            (
              PlayerId(2),
              PlayerInfo {
                player_id: PlayerId(2),
                player_name: PlayerName::from("test2"),
                player_reconnect_key: PlayerReconnectKey(321),
                player_last_seen: 0,
                player_connected: false,
                player_score: 0
              }
            )
          ]),
          round_number: 1
        },
        board: expected_board()
      })
    );
  }

  #[test]
  fn disconnect_close() {
    let events = vec![
      RoomEvent::Connect(Connect {
        player_name: PlayerName::from("test"),
        player_id: PlayerId(1),
        player_reconnect_key: PlayerReconnectKey(123),
      }),
      RoomEvent::SoftDisconnect(Disconnect {
        player_id: PlayerId(1),
      }),
    ];
    let result = simulate(events);
    assert_eq!(result, RoomState::Closed);
  }

  #[test]
  fn start_round() {
    let events = vec![
      RoomEvent::Connect(Connect {
        player_name: PlayerName::from("test"),
        player_id: PlayerId(1),
        player_reconnect_key: PlayerReconnectKey(123),
      }),
      RoomEvent::StartRound,
    ];
    let result = simulate(events);
    assert_eq!(
      result,
      RoomState::RoundStart(RoundStart {
        meta: RoomMeta {
          room_id: expected_room(),
          generator: expected_generator(),
          player_info: HashMap::from([(
            PlayerId(1),
            PlayerInfo {
              player_id: PlayerId(1),
              player_name: PlayerName::from("test"),
              player_reconnect_key: PlayerReconnectKey(123),
              player_last_seen: 1,
              player_connected: true,
              player_score: 0
            }
          )]),
          round_number: 1
        },
        board: expected_board()
      })
    );
  }

  #[test]
  fn bidding_order() {
    let events = vec![
      RoomEvent::Connect(Connect {
        player_name: PlayerName::from("test"),
        player_id: PlayerId(1),
        player_reconnect_key: PlayerReconnectKey(123),
      }),
      RoomEvent::Connect(Connect {
        player_name: PlayerName::from("test2"),
        player_id: PlayerId(2),
        player_reconnect_key: PlayerReconnectKey(123),
      }),
      RoomEvent::Connect(Connect {
        player_name: PlayerName::from("test3"),
        player_id: PlayerId(3),
        player_reconnect_key: PlayerReconnectKey(123),
      }),
      RoomEvent::StartRound,
      RoomEvent::MakeBid(MakeBid {
        player_id: PlayerId(3),
        bid_value: 5,
      }),
      RoomEvent::MakeBid(MakeBid {
        player_id: PlayerId(1),
        bid_value: 5,
      }),
      RoomEvent::MakeBid(MakeBid {
        player_id: PlayerId(2),
        bid_value: 5,
      }),
      RoomEvent::FinalizeBids,
    ];
    let result = simulate(events);

    let expected_meta = RoomMeta {
      room_id: expected_room(),
      generator: expected_generator(),
      player_info: HashMap::from([
        (
          PlayerId(1),
          PlayerInfo {
            player_id: PlayerId(1),
            player_name: PlayerName::from("test"),
            player_reconnect_key: PlayerReconnectKey(123),
            player_last_seen: 1,
            player_connected: true,
            player_score: 0,
          },
        ),
        (
          PlayerId(2),
          PlayerInfo {
            player_id: PlayerId(2),
            player_name: PlayerName::from("test2"),
            player_reconnect_key: PlayerReconnectKey(123),
            player_last_seen: 1,
            player_connected: true,
            player_score: 0,
          },
        ),
        (
          PlayerId(3),
          PlayerInfo {
            player_id: PlayerId(3),
            player_name: PlayerName::from("test3"),
            player_reconnect_key: PlayerReconnectKey(123),
            player_last_seen: 1,
            player_connected: true,
            player_score: 0,
          },
        ),
      ]),
      round_number: 1,
    };
    let mut expected = RoundSolving {
      meta: expected_meta.clone(),
      board: expected_board(),
      player_bids: PlayerBids {
        bids: HashMap::from([
          (PlayerId(1), PlayerBid::Prospective { value: 5, order: 1 }),
          (PlayerId(2), PlayerBid::Prospective { value: 5, order: 2 }),
          (PlayerId(3), PlayerBid::Prospective { value: 5, order: 0 }),
        ]),
        timestamp: 3,
      },
      solver: PlayerId(3),
      solution: Vec::new(),
    };

    assert_eq!(result, RoomState::RoundSolving(expected.clone()));

    let result = simulate_on(result, vec![RoomEvent::YieldSolve]);
    expected
      .player_bids
      .bids
      .insert(PlayerId(3), PlayerBid::Failed { value: 5 });
    expected.solver = PlayerId(1);
    assert_eq!(result, RoomState::RoundSolving(expected.clone()));

    let result = simulate_on(result, vec![RoomEvent::YieldSolve]);
    expected
      .player_bids
      .bids
      .insert(PlayerId(1), PlayerBid::Failed { value: 5 });
    expected.solver = PlayerId(2);
    assert_eq!(result, RoomState::RoundSolving(expected.clone()));

    let result = simulate_on(result, vec![RoomEvent::YieldSolve]);
    assert_eq!(
      result,
      RoomState::RoundSummary(RoundSummary {
        meta: expected_meta.clone(),
        last_round_board: Some(expected_board()),
        last_round_solution: None,
        last_solver: None
      })
    );
  }

  #[test]
  fn solve() {
    let events = vec![
      RoomEvent::Connect(Connect {
        player_name: PlayerName::from("test"),
        player_id: PlayerId(1),
        player_reconnect_key: PlayerReconnectKey(123),
      }),
      RoomEvent::StartRound,
      RoomEvent::MakeBid(MakeBid {
        player_id: PlayerId(1),
        bid_value: 1,
      }),
      RoomEvent::FinalizeBids,
      RoomEvent::UpdateSolution(UpdateSolution {
        solution: vec![SolutionStep {
          actor: 0,
          direction: Direction::Down,
        }],
      }),
    ];
    let result = simulate(events);

    assert_eq!(
      result,
      RoomState::RoundSummary(RoundSummary {
        meta: RoomMeta {
          room_id: expected_room(),
          generator: expected_generator(),
          player_info: HashMap::from([(
            PlayerId(1),
            PlayerInfo {
              player_id: PlayerId(1),
              player_name: PlayerName::from("test"),
              player_reconnect_key: PlayerReconnectKey(123),
              player_last_seen: 1,
              player_connected: true,
              player_score: 1
            }
          )]),
          round_number: 1
        },
        last_round_board: Some(expected_board()),
        last_round_solution: Some(vec![SolutionStep {
          actor: 0,
          direction: Direction::Down,
        }]),
        last_solver: Some(PlayerId(1))
      })
    );
  }
}
