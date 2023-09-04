use std::sync::Arc;

use axum::extract::ws::WebSocket;
use axum::extract::State;
use axum::extract::WebSocketUpgrade;
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use axum::Server;

struct AppState {}

async fn ws_handler(
  ws: WebSocketUpgrade,
  State(state): State<Arc<AppState>>,
) -> Response {
  ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
  while let Some(msg) = socket.recv().await {
    let msg = if let Ok(msg) = msg {
      msg
    } else {
      // client disconnected
      return;
    };

    if socket.send(msg).await.is_err() {
      // client disconnected
      return;
    }
  }
}

#[tokio::main]
async fn main() {
  println!("Starting server...");

  let app_state = Arc::new(AppState {});

  let app = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/ws", get(ws_handler))
    .with_state(app_state);

  Server::bind(&"0.0.0.0:8001".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}
