use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
};
use tokio::sync::broadcast;

use crate::AppState;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> Response {
    ws.on_upgrade(|socket| handle_ws(socket, state))
}

async fn handle_ws(socket: WebSocket, state: Arc<AppState>) {
    let (mut sink, mut stream) = socket.split();
    let socket_id = uuid::Uuid::new_v4().to_string();

    // Track this connection
    state.connected_sockets.insert(socket_id.clone(), ());

    // Send connected event with socket_id
    let connected_msg = serde_json::json!({
        "event": "connected",
        "data": { "socket_id": &socket_id }
    });
    if sink
        .send(Message::Text(connected_msg.to_string().into()))
        .await
        .is_err()
    {
        state.connected_sockets.remove(&socket_id);
        return;
    }

    // Track active subscriptions for this client
    let subscriptions: Arc<tokio::sync::Mutex<Vec<String>>> =
        Arc::new(tokio::sync::Mutex::new(Vec::new()));

    // Channel for forwarding broadcast messages to ws sink
    let (forward_tx, mut forward_rx) = tokio::sync::mpsc::unbounded_channel::<String>();

    // Spawn write loop: forwards from mpsc → ws sink
    let write_handle = tokio::spawn(async move {
        while let Some(msg) = forward_rx.recv().await {
            if sink.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // Read loop: handles subscribe/unsubscribe messages
    use futures_util::StreamExt;
    while let Some(Ok(msg)) = stream.next().await {
        match msg {
            Message::Text(text) => {
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&text) {
                    let action = parsed.get("action").and_then(|a| a.as_str());
                    let channel = parsed.get("channel").and_then(|c| c.as_str());

                    match (action, channel) {
                        (Some("subscribe"), Some(channel_name)) => {
                            let channel_name = channel_name.to_string();
                            let tx = state
                                .channels
                                .entry(channel_name.clone())
                                .or_insert_with(|| broadcast::channel(64).0)
                                .clone();

                            let mut rx = tx.subscribe();
                            let forward_tx_clone = forward_tx.clone();

                            // Spawn a task to forward messages from this channel's
                            // broadcast to the client's mpsc
                            tokio::spawn(async move {
                                while let Ok(msg) = rx.recv().await {
                                    if forward_tx_clone.send(msg).is_err() {
                                        break;
                                    }
                                }
                            });

                            subscriptions.lock().await.push(channel_name);
                        }
                        (Some("unsubscribe"), Some(channel_name)) => {
                            let mut subs = subscriptions.lock().await;
                            subs.retain(|s| s != channel_name);
                        }
                        _ => {}
                    }
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    // Cleanup on disconnect
    state.connected_sockets.remove(&socket_id);
    drop(forward_tx); // This will cause the write loop to end
    let _ = write_handle.await;
}

/// Send a message to all subscribers of a channel.
pub fn trigger(state: &AppState, channel: &str, event: &str, data: serde_json::Value) {
    let msg = serde_json::json!({
        "event": event,
        "channel": channel,
        "data": data
    });
    if let Some(tx) = state.channels.get(channel) {
        // Ignore send errors (no receivers is fine)
        let _ = tx.send(msg.to_string());
    }
}

use futures_util::SinkExt;
