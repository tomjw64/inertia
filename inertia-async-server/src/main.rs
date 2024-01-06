mod difficulty_board_generator;
mod join;
mod state;
mod ws_receiver;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::ws;
use axum::extract::ws::WebSocket;
use axum::extract::ConnectInfo;
use axum::extract::State;
use axum::extract::WebSocketUpgrade;
use axum::response::Response;
use axum::routing::get;
use axum::Json;
use axum::Router;
use axum::Server;
use futures::SinkExt;
use futures::StreamExt;
use inertia_core::message::FromClientMessage;
use inertia_core::message::ToClientMessage;
use inertia_core::state::event::apply_event::RoomEvent;
use inertia_core::state::event::disconnect::Disconnect;
use serde_json::json;
use serde_json::Value;
use sqlx::sqlite::SqlitePoolOptions;
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::join::join;
use crate::join::JoinInfo;

use crate::state::AppState;
use crate::ws_receiver::handle_message_from_client;

const DB_URL: &str = "sqlite:db/boards.db?mode=ro";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  tracing_subscriber::registry()
    .with(
      tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "inertia_async_server=debug".into()),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

  let db_pool = SqlitePoolOptions::new()
    .max_connections(16)
    .connect(&std::env::var("DB_URL").unwrap_or(DB_URL.to_string()))
    .await?;

  let app_state = AppState {
    db_pool,
    rooms: Arc::new(RwLock::new(HashMap::new())),
  };

  let app = Router::new()
    .route("/healthcheck", get(healthcheck))
    .route("/status", get(status))
    .layer(CorsLayer::permissive())
    .route("/ws", get(ws_handler))
    .with_state(app_state);

  let address = SocketAddr::from(([0, 0, 0, 0], 8001));
  tracing::info!("Listening on {}", address);
  Server::bind(&address)
    .serve(app.into_make_service_with_connect_info::<SocketAddr>())
    .await
    .unwrap();
  Ok(())
}

async fn ws_handler(
  ws: WebSocketUpgrade,
  State(state): State<AppState>,
  ConnectInfo(socket_address): ConnectInfo<SocketAddr>,
) -> Response {
  tracing::debug!("WebSocket connect: {}", socket_address);
  ws.on_upgrade(move |socket| handle_socket(socket, socket_address, state))
}

async fn healthcheck() -> &'static str {
  "200 OK"
}

async fn status(State(state): State<AppState>) -> Json<Value> {
  let room_count = state.get_room_count().await;
  Json(json!({ "room_count": room_count }))
}

async fn handle_socket(
  socket: WebSocket,
  socket_address: SocketAddr,
  state: AppState,
) {
  macro_rules! ws_debug {
    ($($t:tt)*) => (tracing::debug!("WebSocket [{}]: {}", socket_address, format_args!($($t)*)))
  }

  let (mut ws_sender, mut ws_receiver) = socket.split();

  let (individual_channel_sender, mut individual_channel_receiver) =
    mpsc::channel::<ToClientMessage>(16);

  let JoinInfo {
    room_id,
    player_id,
    player_name,
    mut broadcast_channel_receiver,
    ..
  } = match join(&mut ws_receiver, &socket_address, &state).await {
    Ok(info) => info,
    Err(err) => {
      ws_debug!("Failed to connect: {:?}", err);
      return;
    }
  };

  ws_debug!(
    "Connected. Room: {}. Player: {:?}",
    room_id.0,
    player_name.0
  );

  state.broadcast_room(room_id).await.ok();

  let mut individual_channel_receive_task = tokio::spawn(async move {
    loop {
      let channel_msg = match individual_channel_receiver.recv().await {
        Some(channel_msg) => channel_msg,
        None => {
          ws_debug!("Individual channel closed");
          break;
        }
      };
      let msg_json = match serde_json::to_string(&channel_msg) {
        Ok(msg_json) => msg_json,
        Err(err) => {
          ws_debug!("Failed to serialize channel message: {}", err);
          continue;
        }
      };
      match ws_sender.send(ws::Message::Text(msg_json)).await {
        Ok(_) => continue,
        Err(err) => {
          ws_debug!("Failed to send WS message: {}", err);
          break;
        }
      }
    }
  });

  let individual_sender_for_task = individual_channel_sender.clone();
  let mut broadcast_channel_receive_task = tokio::spawn(async move {
    loop {
      let channel_msg = broadcast_channel_receiver.recv().await;
      let channel_msg = match channel_msg {
        Ok(channel_msg) => channel_msg,
        Err(err) => match err {
          broadcast::error::RecvError::Closed => {
            ws_debug!("Broadcast channel closed");
            break;
          }
          broadcast::error::RecvError::Lagged(_) => {
            ws_debug!("Messages from broadcast channel lagged");
            continue;
          }
        },
      };
      if individual_sender_for_task.send(channel_msg).await.is_err() {
        ws_debug!("Failed to forward message to individual channel");
        break;
      }
    }
  });

  let individual_sender_for_task = individual_channel_sender.clone();
  let state_clone_for_task = state.clone();
  let mut ws_receive_task = tokio::spawn(async move {
    loop {
      let ws_msg = match ws_receiver.next().await {
        Some(ws_msg) => ws_msg,
        None => {
          ws_debug!("End of WS message stream");
          break;
        }
      };
      let ws_msg = match ws_msg {
        Ok(msg) => msg,
        Err(err) => {
          ws_debug!("Error from receiver: {}", err);
          continue;
        }
      };
      let ws_msg = match ws_msg {
        ws::Message::Text(text) => text,
        ws::Message::Close(_) => {
          ws_debug!("WS closed");
          break;
        }
        _ => continue,
      };
      let ws_msg = match serde_json::from_str::<FromClientMessage>(&ws_msg) {
        Ok(msg) => msg,
        Err(err) => {
          ws_debug!(
            "Failed to parse message. Message: {:?}, Error: {}",
            ws_msg,
            err
          );
          continue;
        }
      };
      ws_debug!("Received message: {:?}", ws_msg);
      if let Err(error) = handle_message_from_client(
        ws_msg,
        &state_clone_for_task,
        room_id,
        player_id,
        &individual_sender_for_task,
      )
      .await
      {
        ws_debug!("Error: {}", error);
      };
    }
  });

  tokio::select! {
    _ = (&mut individual_channel_receive_task) => {
      broadcast_channel_receive_task.abort();
      ws_receive_task.abort();
    }
    _ = (&mut broadcast_channel_receive_task) => {
      individual_channel_receive_task.abort();
      ws_receive_task.abort();
    },
    _ = (&mut ws_receive_task) => {
      individual_channel_receive_task.abort();
      broadcast_channel_receive_task.abort();
    },
  };
  ws_debug!("Disconnecting");

  let disconect_event = RoomEvent::SoftDisconnect(Disconnect { player_id });
  state.apply_event(room_id, disconect_event).await.ok();
  state.clean_up_room(room_id).await;
}
