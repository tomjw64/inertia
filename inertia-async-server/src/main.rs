use std::collections::HashMap;
use std::collections::HashSet;
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
use inertia_core::message::JoinMessage;
use inertia_core::state::RoomData;
use inertia_core::state::RoomId;
use inertia_core::state::RoomState;
use tokio::sync::broadcast;
use tokio::sync::Mutex;
use tokio::sync::RwLock;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

struct Room {
  channel: broadcast::Sender<String>,
  room_data: Mutex<RoomData>,
}

impl Room {
  fn new(room_id: RoomId) -> Self {
    Room {
      channel: broadcast::channel(16).0,
      room_data: Mutex::new(RoomData {
        room_id,
        players: HashMap::new(),
        player_reconnect_keys: HashMap::new(),
        players_connected: HashMap::new(),
        player_scores: HashMap::new(),
        round_number: 0,
        state: RoomState::Lobby,
      }),
    }
  }
}

struct AppState {
  rooms: RwLock<HashMap<RoomId, Room>>,
}

impl AppState {}

async fn ws_handler(
  ws: WebSocketUpgrade,
  State(state): State<Arc<AppState>>,
  ConnectInfo(socket_address): ConnectInfo<SocketAddr>,
) -> Response {
  tracing::debug!("WebSocket connect: {}", socket_address);
  ws.on_upgrade(move |socket| handle_socket(socket, socket_address, state))
}

async fn broadcast_room_data(state: &Arc<AppState>, room_id: &RoomId) {
  let rooms = state.rooms.read().await;
  let room = match rooms.get(&room_id) {
    Some(room) => room,
    None => {
      tracing::debug!(
        "Could not broadcast room data for room {}. Room disappeared.",
        room_id.0
      );
      return;
    }
  };
  let room_data = room.room_data.lock().await;
  room
    .channel
    .send(serde_json::to_string(&*room_data).unwrap())
    .ok();
}

async fn handle_socket(
  socket: WebSocket,
  socket_address: SocketAddr,
  state: Arc<AppState>,
) {
  let (mut ws_sender, mut ws_receiver) = socket.split();

  let (
    room_id,
    player_id,
    player_reconnect_key,
    player_name,
    channel_sender,
    mut channel_receiver,
  ) = {
    loop {
      let msg = ws_receiver.next().await;
      let msg = match msg {
        None => {
          tracing::debug!(
            "WebSocket [{}] disconnected. No message received.",
            socket_address
          );
          return;
        }
        Some(msg) => msg,
      };
      let msg = match msg {
        Err(error) => {
          tracing::debug!(
            "WebSocket [{}] disconnected. Error: {}",
            socket_address,
            error
          );
          return;
        }
        Ok(msg) => msg,
      };
      let msg = match msg {
        Message::Text(msg) => msg,
        _ => {
          tracing::debug!(
            "WebSocket [{}] join rejected. Unexpected message: {:?}",
            socket_address,
            msg
          );
          continue;
        }
      };

      let join_message = match serde_json::from_str::<JoinMessage>(&msg) {
        Ok(join_message) => join_message,
        Err(error) => {
          tracing::debug!(
            "WebSocket [{}] join rejected. Failed to parse message. Message: {:?}, Error: {}",
            socket_address,
            msg,
            error
          );
          continue;
        }
      };

      if join_message.player_name.0.is_empty() {
        tracing::debug!(
          "WebSocket [{}] join rejected. Empty username.",
          socket_address
        );
        continue;
      }

      if join_message.room_id.0 == 0 {
        tracing::debug!(
          "WebSocket [{}] join rejected. Room id 0.",
          socket_address
        );
        continue;
      }

      let should_create_room = state
        .rooms
        .read()
        .await
        .get(&join_message.room_id)
        .is_none();

      if should_create_room {
        state
          .rooms
          .write()
          .await
          .entry(join_message.room_id)
          .or_insert_with(|| Room::new(join_message.room_id));
      }

      let rooms = state.rooms.read().await;
      let room = match rooms.get(&join_message.room_id) {
        Some(room) => room,
        None => {
          tracing::debug!(
            "WebSocket [{}] join rejected. Room disappeared.",
            socket_address
          );
          continue;
        }
      };

      let mut room_data = room.room_data.lock().await;

      let reconnect_key_for_id =
        room_data.player_reconnect_keys.get(&join_message.player_id);
      let id_for_username = room_data
        .players
        .iter()
        .find(|&(_, player_name)| player_name == &join_message.player_name)
        .map(|(id, _)| id);

      if id_for_username.is_some()
        && id_for_username != Some(&join_message.player_id)
      {
        tracing::debug!(
          "WebSocket [{}] join rejected. Username taken: {}.",
          socket_address,
          join_message.player_name.0
        );
        continue;
      }

      if reconnect_key_for_id.is_some()
        && reconnect_key_for_id != Some(&join_message.player_reconnect_key)
      {
        tracing::debug!(
          "WebSocket [{}] join rejected. Bad reconnect key: {}.",
          socket_address,
          join_message.player_reconnect_key.0
        );
        continue;
      }

      room_data
        .players
        .insert(join_message.player_id, join_message.player_name.clone());
      room_data
        .players_connected
        .insert(join_message.player_id, true);
      room_data
        .player_reconnect_keys
        .insert(join_message.player_id, join_message.player_reconnect_key);

      break (
        join_message.room_id,
        join_message.player_id,
        join_message.player_reconnect_key,
        join_message.player_name.clone(),
        room.channel.clone(),
        room.channel.subscribe(),
      );
    }
  };

  tracing::debug!(
    "WebSocket [{}] connected. Room: {}. Player: {:?}.",
    socket_address,
    room_id.0,
    player_name.0,
  );

  broadcast_room_data(&state, &room_id).await;

  let mut send_task = tokio::spawn(async move {
    loop {
      let channel_msg = channel_receiver.recv().await;
      let channel_msg = match channel_msg {
        Ok(channel_msg) => channel_msg,
        Err(err) => {
          tracing::debug!(
            "WebSocket [{}] receiver error: {}",
            socket_address,
            err
          );
          continue;
        }
      };
      if ws_sender.send(Message::Text(channel_msg)).await.is_err() {
        break;
      }
    }
  });

  let mut receive_task = tokio::spawn(async move {
    // TODO: Handle None and Err cases explicitly with trace logging
    while let Some(Ok(Message::Text(text))) = ws_receiver.next().await {
      tracing::debug!(
        "Received message from WebSocket [{}]: {}",
        socket_address,
        text
      );
    }
  });

  tokio::select! {
    _ = (&mut send_task) => receive_task.abort(),
    _ = (&mut receive_task) => send_task.abort(),
  };

  tracing::debug!("Disconnecting WebSocket [{}].", socket_address);

  let should_delete_room = {
    let rooms = state.rooms.read().await;
    let room = match rooms.get(&room_id) {
      Some(room) => room,
      None => {
        tracing::debug!(
          "WebSocket [{}] disconnect cleanup aborted. Room disappeared.",
          socket_address
        );
        return;
      }
    };
    let mut room_data = room.room_data.lock().await;
    room_data.players_connected.remove(&player_id);
    if let RoomState::Lobby = room_data.state {
      room_data.players.remove(&player_id);
      room_data.player_reconnect_keys.remove(&player_id);
      room_data.player_scores.remove(&player_id);
    }
    room_data.players_connected.is_empty()
  };

  broadcast_room_data(&state, &room_id).await;

  if should_delete_room {
    state.rooms.write().await.remove(&room_id);
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
