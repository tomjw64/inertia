use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

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
use inertia_core::state::RoomData;
use inertia_core::state::RoomId;
use tokio::sync::broadcast::Sender;
use tokio::sync::RwLock;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

struct Room {
  channel: Sender<String>,
  room_data: RwLock<RoomData>,
}

struct AppState {
  rooms: RwLock<HashMap<RoomId, Room>>,
}

async fn ws_handler(
  ws: WebSocketUpgrade,
  State(state): State<Arc<AppState>>,
  ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Response {
  tracing::debug!("Initiating WebSocket connection: {}", addr);
  ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
  let (mut sender, mut receiver) = socket.split();
  while let Some(msg) = receiver.next().await {
    let msg = if let Ok(msg) = msg {
      msg
    } else {
      // client disconnected
      return;
    };

    if sender.send(msg).await.is_err() {
      // client disconnected
      return;
    }
  }
}

#[tokio::main]
async fn main() {
  tracing_subscriber::registry()
    .with(
      tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "inertia_async_server=trace".into()),
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

  let address = SocketAddr::from(([127, 0, 0, 1], 8000));
  tracing::info!("Listening on {}", address);
  Server::bind(&address)
    .serve(app.into_make_service_with_connect_info::<SocketAddr>())
    .await
    .unwrap();
}
