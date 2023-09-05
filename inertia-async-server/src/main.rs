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
        players_connected: HashSet::new(),
        player_scores: HashMap::new(),
        round_number: 0,
        data_version: 0,
        state: RoomState::Lobby,
      }),
    }
  }
}

struct AppState {
  rooms: RwLock<HashMap<RoomId, Room>>,
}

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

      let must_create_room = {
        let rooms = state.rooms.read().await;
        rooms.get(&join_message.room_id).is_none()
      };

      if must_create_room {
        let mut rooms = state.rooms.write().await;
        rooms
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
      room_data.players_connected.insert(join_message.player_id);
      room_data
        .player_reconnect_keys
        .insert(join_message.player_id, join_message.player_reconnect_key);
      room_data.data_version += 1;

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

  tracing::debug!("WebSocket [{}] connected.", socket_address);

  {
    let rooms = state.rooms.read().await;
    let room = match rooms.get(&room_id) {
      Some(room) => room,
      None => {
        tracing::debug!(
          "Disconnecting WebSocket [{}]. Room disappeared.",
          socket_address
        );
        return;
      }
    };
    let room_data = room.room_data.lock().await;
    channel_sender
      .send(serde_json::to_string(&*room_data).unwrap())
      .ok();
  }

  let mut send_task = tokio::spawn(async move {
    while let Ok(msg) = channel_receiver.recv().await {
      // In any websocket error, break loop.
      if ws_sender.send(Message::Text(msg)).await.is_err() {
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

  // TODO: Remove player from connected players.
  // TODO: Remove Room if room empty after disconnect.
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
