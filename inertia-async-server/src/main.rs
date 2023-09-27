mod join;
mod state;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::ws::Message;
use axum::extract::ws::WebSocket;
use axum::extract::ConnectInfo;
use axum::extract::State;
use axum::extract::WebSocketUpgrade;
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use axum::Server;
use futures::SinkExt;
use futures::StreamExt;
use inertia_core::message::FromClientMessage;
use inertia_core::state::PlayerId;
use inertia_core::state::RoomId;
use tokio::sync::RwLock;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::join::join;
use crate::join::JoinInfo;

use crate::state::AppState;

async fn ws_handler(
  ws: WebSocketUpgrade,
  State(state): State<Arc<AppState>>,
  ConnectInfo(socket_address): ConnectInfo<SocketAddr>,
) -> Response {
  tracing::debug!("WebSocket connect: {}", socket_address);
  ws.on_upgrade(move |socket| handle_socket(socket, socket_address, state))
}

async fn handle_socket(
  socket: WebSocket,
  socket_address: SocketAddr,
  state: Arc<AppState>,
) {
  macro_rules! ws_debug {
    ($($t:tt)*) => (tracing::debug!("WebSocket [{}]: {}", socket_address, format_args!($($t)*)))
  }

  let (mut ws_sender, mut ws_receiver) = socket.split();

  let JoinInfo {
    room_id,
    player_id,
    player_name,
    mut channel_sender,
    mut channel_receiver,
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

  let mut send_task = tokio::spawn(async move {
    loop {
      let channel_msg = channel_receiver.recv().await;
      let channel_msg = match channel_msg {
        Ok(channel_msg) => channel_msg,
        Err(err) => {
          ws_debug!("Receiver error: {}", err);
          continue;
        }
      };
      if ws_sender.send(Message::Text(channel_msg)).await.is_err() {
        break;
      }
    }
  });

  let mut receive_task = tokio::spawn(async move {
    while let Some(ws_msg) = ws_receiver.next().await {
      let ws_msg = match ws_msg {
        Ok(msg) => msg,
        Err(err) => {
          ws_debug!("Error from receiver: {}", err);
          continue;
        }
      };
      let ws_msg = match ws_msg {
        Message::Text(text) => text,
        Message::Close(_) => break,
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
    }
  });

  tokio::select! {
    _ = (&mut send_task) => receive_task.abort(),
    _ = (&mut receive_task) => send_task.abort(),
  };
  ws_debug!("Disconnecting.");

  let should_delete_room = state
    .soft_remove_player(room_id, player_id)
    .await
    .unwrap_or(false);

  if should_delete_room {
    state.rooms.write().await.remove(&room_id);
  } else {
    state.broadcast_room(room_id).await.ok();
  }
}

async fn handle_message_from_client(
  socket_address: SocketAddr,
  msg: FromClientMessage,
  state: &Arc<AppState>,
  room_id: RoomId,
  player_id: PlayerId,
) {
  macro_rules! ws_debug {
    ($($t:tt)*) => (tracing::debug!("WebSocket [{}]: {}", socket_address, format_args!($($t)*)))
  }

  match msg {
    FromClientMessage::Rename(_) => todo!(),
    FromClientMessage::Join(_) => ws_debug!("Unexpected join message."),
    FromClientMessage::StartGame => todo!(),
  }
}

#[tokio::main]
async fn main() {
  tracing_subscriber::registry()
    .with(
      tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "inertia_async_server=debug".into()),
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

  let app_state = Arc::new(AppState {
    rooms: RwLock::new(HashMap::new()),
  });

  let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/ws", get(ws_handler))
    .with_state(app_state);

  let address = SocketAddr::from(([127, 0, 0, 1], 8001));
  tracing::info!("Listening on {}", address);
  Server::bind(&address)
    .serve(app.into_make_service_with_connect_info::<SocketAddr>())
    .await
    .unwrap();
}
