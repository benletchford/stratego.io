use leptos::prelude::*;
use stratego::models::{Board, Cell, Piece};

use super::grid::{Grid, LastMoveInfo, MoveEvent};
use super::loading::Loading;
use crate::ws::{sleep_ms, ConnectionState, ConnectionStateSignal, WsClient};

/// Parse the server's last_move JSON into our LastMoveInfo struct.
fn parse_last_move(val: &serde_json::Value) -> Option<LastMoveInfo> {
    let from = val.get("from")?;
    let to = val.get("to")?;

    let from_pos = from.get("position")?;
    let to_pos = to.get("position")?;

    let from_x = from_pos.get("x")?.as_u64()? as usize;
    let from_y = from_pos.get("y")?.as_u64()? as usize;
    let to_x = to_pos.get("x")?.as_u64()? as usize;
    let to_y = to_pos.get("y")?.as_u64()? as usize;

    let move_type = val
        .get("type")
        .and_then(|t| t.as_str())
        .unwrap_or("move")
        .to_string();

    let from_piece = from
        .get("piece")
        .and_then(|p| serde_json::from_value::<Piece>(p.clone()).ok());
    let to_piece = to
        .get("piece")
        .and_then(|p| serde_json::from_value::<Piece>(p.clone()).ok());

    Some(LastMoveInfo {
        from_x,
        from_y,
        to_x,
        to_y,
        move_type,
        from_piece,
        to_piece,
    })
}

/// Parse a board from a JSON value.
fn parse_board(val: &serde_json::Value) -> Board {
    serde_json::from_value(val.clone()).unwrap_or_else(|_| vec![vec![Cell::Empty(0); 10]; 10])
}

/// The main play page component.
/// Handles two flows:
/// 1. Normal play (hash is a player_hash) - loads game via API
/// 2. Pool mode (hash is "pool") - joins matchmaking pool via WebSocket
#[component]
pub fn PlayPage(#[prop(into)] hash: String) -> impl IntoView {
    let hash_stored = StoredValue::new(hash);

    // State signals
    let (board, set_board) = signal(vec![vec![Cell::Empty(0); 10]; 10]);
    let (side, set_side) = signal(0_i32);
    let (_game_state, set_game_state) = signal(0_i32);
    let (player_hash, set_player_hash) = signal(String::new());
    let (last_move_val, set_last_move) = signal(Option::<LastMoveInfo>::None);
    let (loading, set_loading) = signal(true);
    let (loading_msg, set_loading_msg) = signal("Loading game...".to_string());

    // Move signal: Grid writes move events here
    let (move_event, set_move_event) = signal(Option::<MoveEvent>::None);

    // Function to update game state from server response
    let update_game = move |game: &serde_json::Value| {
        if let Some(b) = game.get("board") {
            set_board.set(parse_board(b));
        }
        if let Some(s) = game.get("side") {
            set_side.set(s.as_i64().unwrap_or(0) as i32);
        }
        if let Some(gs) = game.get("game_state") {
            set_game_state.set(gs.as_i64().unwrap_or(0) as i32);
        }
        if let Some(ph) = game.get("player_hash").and_then(|v| v.as_str()) {
            set_player_hash.set(ph.to_string());
        }

        let lm = game.get("last_move").and_then(|v| {
            if v.is_null() {
                None
            } else {
                parse_last_move(v)
            }
        });
        set_last_move.set(lm);
    };

    // Watch for move events from the grid and send API calls
    Effect::new(move |_| {
        if let Some((fx, fy, tx, ty)) = move_event.get() {
            let ph = player_hash.get_untracked();
            let s = side.get_untracked();
            let from = stratego::models::Position { x: fx, y: fy };
            let to = stratego::models::Position { x: tx, y: ty };

            wasm_bindgen_futures::spawn_local(async move {
                match crate::api::make_move(&ph, s, &from, &to).await {
                    Ok(game) => {
                        update_game(&game);
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Move error: {}", e).into());
                    }
                }
            });
            set_move_event.set(None);
        }
    });

    // Must be called in component scope (not inside spawn_local)
    let navigate = leptos_router::hooks::use_navigate();
    let conn_writer = use_context::<WriteSignal<ConnectionState>>();

    // Refetch game state on WebSocket reconnect (catches missed events)
    let conn_signal = use_context::<ConnectionStateSignal>();
    let (had_connection, set_had_connection) = signal(false);
    Effect::new(move |_| {
        let state = conn_signal
            .as_ref()
            .map(|s| s.0.get())
            .unwrap_or(ConnectionState::Idle);
        match state {
            ConnectionState::Connected => {
                if had_connection.get_untracked() {
                    // This is a REconnect — reset the flag
                    set_had_connection.set(false);
                    let ph = player_hash.get_untracked();
                    if !ph.is_empty() {
                        // Normal game mode — refetch game state
                        wasm_bindgen_futures::spawn_local(async move {
                            if let Ok(game) = crate::api::get_game(&ph).await {
                                update_game(&game);
                            }
                        });
                    } else if hash_stored.get_value() == "pool" {
                        // Pool mode — socket_id changed, must re-join pool.
                        // Reload to restart the flow with the new connection.
                        let win = web_sys::window().unwrap();
                        let _ = win.location().reload();
                    }
                }
            }
            ConnectionState::Disconnected | ConnectionState::Reconnecting(_) => {
                set_had_connection.set(true);
            }
            _ => {}
        }
    });

    // Load game and set up WebSocket
    let hash_val = hash_stored.get_value();
    if hash_val == "pool" {
        // Pool mode
        set_loading_msg.set("Connecting to pool...".to_string());

        let nav1 = navigate.clone();
        let nav2 = navigate.clone();
        let nav3 = navigate.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let ws_client = WsClient::connect(conn_writer);

            // Wait for socket_id
            let mut attempts = 0;
            while ws_client.socket_id().is_none() && attempts < 50 {
                sleep_ms(100).await;
                attempts += 1;
            }

            let socket_id = match ws_client.socket_id() {
                Some(id) => id,
                None => {
                    set_loading_msg.set("Failed to connect to WebSocket.".to_string());
                    return;
                }
            };

            set_loading_msg.set("Connected to pool, setting up match...".to_string());

            // Subscribe to pool channel
            let channel_name = format!("pool-{}", socket_id);
            ws_client.subscribe(&channel_name);

            // Bind opponent_found event (for the first player waiting in pool)
            ws_client.bind(&channel_name, "opponent_found", move |data| {
                if let Some(ph) = data.get("player_hash").and_then(|v| v.as_str()) {
                    nav1(&format!("/play/{}", ph), Default::default());
                }
            });

            // Get saved board from localStorage (saved by setup page)
            let board_json = {
                let win = web_sys::window().unwrap();
                win.local_storage()
                    .ok()
                    .flatten()
                    .and_then(|s| s.get_item("lastBoard").ok().flatten())
            };
            let board_json = match board_json {
                Some(b) if !b.is_empty() && b != "[]" => b,
                _ => {
                    // No board setup — redirect to setup page
                    nav3("/setup/pool", Default::default());
                    return;
                }
            };

            // Join the pool
            match crate::api::join_pool(&board_json, &socket_id).await {
                Ok(Some(player_hash)) => {
                    nav2(&format!("/play/{}", player_hash), Default::default());
                }
                Ok(None) => {
                    set_loading_msg.set("In pool, waiting for an opponent...".to_string());
                }
                Err(e) => {
                    set_loading_msg
                        .set(format!("Failed to join pool: {}", e));
                }
            }

            // Keep ws_client alive (don't drop it)
            std::mem::forget(ws_client);
        });
    } else {
        // Normal play mode - load game
        let hash_clone = hash_val.clone();
        wasm_bindgen_futures::spawn_local(async move {
            set_loading_msg.set("Loading game...".to_string());

            match crate::api::get_game(&hash_clone).await {
                Ok(game) => {
                    update_game(&game);

                    set_loading_msg.set("Connecting to websocket...".to_string());

                    // Connect WebSocket
                    let ws_client = WsClient::connect(conn_writer);

                    // Wait for connection and connected event
                    let mut attempts = 0;
                    while ws_client.socket_id().is_none() && attempts < 50 {
                        sleep_ms(100).await;
                        attempts += 1;
                    }

                    let ph = game
                        .get("player_hash")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();

                    // Subscribe to game channel
                    let channel_name = format!("game-{}", ph);
                    ws_client.subscribe(&channel_name);

                    // Handle game state
                    let gs = game
                        .get("game_state")
                        .and_then(|v| v.as_i64())
                        .unwrap_or(0);

                    if gs == 0 {
                        // Waiting for opponent
                        let join_url = {
                            let win = web_sys::window().unwrap();
                            let loc = win.location();
                            let protocol = loc.protocol().unwrap_or_default();
                            let host = loc.host().unwrap_or_default();
                            let jh = game
                                .get("join_hash")
                                .and_then(|v| v.as_str())
                                .unwrap_or("");
                            format!("{}//{}/setup/join/{}", protocol, host, jh)
                        };
                        set_loading_msg.set(format!(
                            "Waiting for opponent...<br /><br /> {}",
                            join_url
                        ));

                        // Bind blue_ready event
                        let channel_for_bind = channel_name.clone();
                        let ws_ref = ws_client.clone();
                        ws_client.bind(&channel_name, "blue_ready", move |_data| {
                            set_loading.set(false);
                            ws_ref.unbind(&channel_for_bind, "blue_ready");

                            // Refresh game state
                            let ph_clone = ph.clone();
                            wasm_bindgen_futures::spawn_local(async move {
                                if let Ok(game) = crate::api::get_game(&ph_clone).await {
                                    update_game(&game);
                                }
                            });
                        });
                    } else {
                        // Game is ready
                        set_loading.set(false);
                    }

                    // Bind update event for ongoing game updates
                    let ph_for_update = hash_clone.clone();
                    ws_client.bind(&channel_name, "update", move |_data| {
                        let ph_clone = ph_for_update.clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            if let Ok(game) = crate::api::get_game(&ph_clone).await {
                                update_game(&game);
                            }
                        });
                    });

                    // Keep ws_client alive
                    std::mem::forget(ws_client);
                }
                Err(e) => {
                    set_loading_msg.set(e);
                }
            }
        });
    }

    let board_signal: Signal<Board> = board.into();
    let side_signal: Signal<i32> = side.into();
    let last_move_signal: Signal<Option<LastMoveInfo>> = last_move_val.into();
    let loading_msg_signal: Signal<String> = loading_msg.into();

    view! {
        <div class="game-view">
            <Show when=move || loading.get()>
                <Loading message=loading_msg_signal />
            </Show>
            <Show when=move || !loading.get()>
                <div class="grid-container">
                    {(0..9).map(|i| view! {
                        <div class="horizontal-grid-line" data-number=i.to_string()></div>
                    }).collect::<Vec<_>>()}
                    {(0..9).map(|i| view! {
                        <div class="vertical-grid-line" data-number=i.to_string()></div>
                    }).collect::<Vec<_>>()}

                    <Grid
                        board=board_signal
                        side=side_signal
                        last_move=last_move_signal
                        set_move=set_move_event
                    />
                </div>
            </Show>
        </div>
    }
}
