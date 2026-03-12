use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    Form,
};
use serde::Deserialize;
use stratego::{
    board_utils,
    constants::MoveType,
    game_logic,
    models::{Cell, Game, Position},
};

use tokio::time::Instant;

use crate::AppState;

// ----- Create -----

#[derive(Deserialize)]
pub struct CreateForm {
    board: String,
}

pub async fn create_handler(
    State(state): State<Arc<AppState>>,
    Form(form): Form<CreateForm>,
) -> impl IntoResponse {
    let mut setup: Vec<Vec<Cell>> = match serde_json::from_str(&form.board) {
        Ok(s) => s,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid board JSON").into_response(),
    };

    if setup.len() != 4 || setup.iter().any(|row| row.len() != 10) {
        return (StatusCode::BAD_REQUEST, "Board must be 4 rows of 10").into_response();
    }

    // Set all pieces to side 0 (red)
    for row in &mut setup {
        for cell in row {
            if let Cell::Piece(p) = cell {
                p.side = 0;
            }
        }
    }

    let mut game = Game {
        id: uuid::Uuid::new_v4().to_string(),
        red_hash: uuid::Uuid::new_v4().hex_string_short(),
        blue_hash: uuid::Uuid::new_v4().hex_string_short(),
        join_hash: uuid::Uuid::new_v4().hex_string_short(),
        board: Game::empty_board(),
        red_setup: None,
        blue_setup: None,
        moves: vec![],
        turn: false, // red goes first
        private: true,
        game_state: 0, // WAITING_FOR_OPPONENT
        created: chrono::Utc::now().to_rfc3339(),
        modified: chrono::Utc::now().to_rfc3339(),
    };

    game.set_red_setup(&setup);
    game.set_blocks();

    if let Err(e) = state.storage.save_game(&game).await {
        tracing::error!("Failed to save game: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to save game").into_response();
    }

    let game_dict = board_utils::get_sendable_game(&game, 0);
    Json(game_dict).into_response()
}

// ----- Join -----

#[derive(Deserialize)]
pub struct JoinForm {
    join_hash: String,
    board: String,
}

pub async fn join_handler(
    State(state): State<Arc<AppState>>,
    Form(form): Form<JoinForm>,
) -> impl IntoResponse {
    let mut setup: Vec<Vec<Cell>> = match serde_json::from_str(&form.board) {
        Ok(s) => s,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid board JSON").into_response(),
    };

    if setup.len() != 4 || setup.iter().any(|row| row.len() != 10) {
        return (StatusCode::BAD_REQUEST, "Board must be 4 rows of 10").into_response();
    }

    // Set all pieces to side 1 (blue)
    for row in &mut setup {
        for cell in row {
            if let Cell::Piece(p) = cell {
                p.side = 1;
            }
        }
    }

    // Find game by join_hash
    let game_id = match state.storage.get_game_id_by_hash("join", &form.join_hash).await {
        Ok(id) => id,
        Err(_) => return (StatusCode::NOT_FOUND, "Game not found").into_response(),
    };

    // Lock the game
    let lock = state.get_game_lock(&game_id);
    let _guard = lock.lock().await;

    let mut game = match state.storage.load_game_by_id(&game_id).await {
        Ok(g) => g,
        Err(_) => return (StatusCode::NOT_FOUND, "Game not found").into_response(),
    };

    if game.game_state != 0 {
        return (StatusCode::BAD_REQUEST, "Game already has two players").into_response();
    }

    game.set_blue_setup(&setup);
    game.game_state = 1; // READY
    game.modified = chrono::Utc::now().to_rfc3339();

    if let Err(e) = state.storage.save_game(&game).await {
        tracing::error!("Failed to save game: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to save").into_response();
    }

    // Red player will detect the opponent joined via polling (modified timestamp changes)
    let game_dict = board_utils::get_sendable_game(&game, 1);
    Json(game_dict).into_response()
}

// ----- Move -----

#[derive(Deserialize)]
pub struct MoveForm {
    player_hash: String,
    side: String,
    from: String,
    to: String,
}

pub async fn move_handler(
    State(state): State<Arc<AppState>>,
    Form(form): Form<MoveForm>,
) -> impl IntoResponse {
    let side: i32 = match form.side.parse() {
        Ok(s) => s,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid side").into_response(),
    };

    let mut from_pos: Position = match serde_json::from_str(&form.from) {
        Ok(p) => p,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid from position").into_response(),
    };

    let mut to_pos: Position = match serde_json::from_str(&form.to) {
        Ok(p) => p,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid to position").into_response(),
    };

    // Find the game
    let game_id = if side == 0 {
        state.storage.get_game_id_by_hash("red", &form.player_hash).await
    } else {
        state.storage.get_game_id_by_hash("blue", &form.player_hash).await
    };

    let game_id = match game_id {
        Ok(id) => id,
        Err(_) => return (StatusCode::NOT_FOUND, "Game not found").into_response(),
    };

    // Lock the game
    let lock = state.get_game_lock(&game_id);
    let _guard = lock.lock().await;

    let mut game = match state.storage.load_game_by_id(&game_id).await {
        Ok(g) => g,
        Err(_) => return (StatusCode::NOT_FOUND, "Game not found").into_response(),
    };

    // Turn validation: red moves when turn=false, blue moves when turn=true
    let is_red = game.red_hash == form.player_hash;
    let is_blue = game.blue_hash == form.player_hash;
    if (is_red && game.turn) || (is_blue && !game.turn) {
        return (StatusCode::UNAUTHORIZED, "Not your turn").into_response();
    }

    if game.has_ended() {
        return (StatusCode::UNAUTHORIZED, "Game has ended").into_response();
    }

    // Reverse positions for blue
    if side == 1 {
        board_utils::reverse_position(&mut from_pos);
        board_utils::reverse_position(&mut to_pos);
    }

    // Validate move
    let move_type = match game_logic::check_move(&game, &from_pos, &to_pos) {
        Ok(mt) => mt,
        Err(e) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "message": e.to_string() })),
            )
                .into_response();
        }
    };

    // Apply move
    match move_type {
        MoveType::Move => {
            game.move_piece(&from_pos, &to_pos);
            game.flip_turn();
            game.set_last_move(serde_json::json!({
                "type": "move",
                "from": { "position": from_pos },
                "to": { "position": to_pos }
            }));
        }
        MoveType::AttackWon => {
            let from_piece = game.get_piece(&from_pos).clone();
            let to_piece = game.get_piece(&to_pos).clone();
            game.move_piece(&from_pos, &to_pos);
            game.flip_turn();
            game.set_last_move(serde_json::json!({
                "type": "won",
                "from": { "piece": from_piece, "position": from_pos },
                "to": { "piece": to_piece, "position": to_pos }
            }));
        }
        MoveType::AttackLost => {
            let from_piece = game.get_piece(&from_pos).clone();
            let to_piece = game.get_piece(&to_pos).clone();
            game.delete_piece(&from_pos);
            game.flip_turn();
            game.set_last_move(serde_json::json!({
                "type": "lost",
                "from": { "piece": from_piece, "position": from_pos },
                "to": { "piece": to_piece, "position": to_pos }
            }));
        }
        MoveType::AttackDraw => {
            let from_piece = game.get_piece(&from_pos).clone();
            let to_piece = game.get_piece(&to_pos).clone();
            game.delete_piece(&from_pos);
            game.delete_piece(&to_pos);
            game.flip_turn();
            game.set_last_move(serde_json::json!({
                "type": "draw",
                "from": { "piece": from_piece, "position": from_pos },
                "to": { "piece": to_piece, "position": to_pos }
            }));
        }
        MoveType::Capture => {
            let from_piece = game.get_piece(&from_pos).clone();
            let to_piece = game.get_piece(&to_pos).clone();
            game.move_piece(&from_pos, &to_pos);
            // Note: no flip_turn on capture (game is over)
            game.set_last_move(serde_json::json!({
                "type": "capture",
                "from": { "piece": from_piece, "position": from_pos },
                "to": { "piece": to_piece, "position": to_pos }
            }));
        }
    }

    game.modified = chrono::Utc::now().to_rfc3339();

    if let Err(e) = state.storage.save_game(&game).await {
        tracing::error!("Failed to save game: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to save").into_response();
    }

    // Opponent will detect the move via polling (modified timestamp changes)
    let game_dict = board_utils::get_sendable_game(&game, side);
    Json(game_dict).into_response()
}

// ----- Game (GET) -----

#[derive(Deserialize)]
pub struct GameQuery {
    player_hash: String,
}

pub async fn game_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<GameQuery>,
) -> impl IntoResponse {
    let (game, side) = match state.storage.load_game_by_player_hash(&query.player_hash).await {
        Ok(gs) => gs,
        Err(_) => return (StatusCode::NOT_FOUND, "Game not found").into_response(),
    };

    let game_dict = board_utils::get_sendable_game(&game, side);
    Json(game_dict).into_response()
}

// ----- Pool Join -----

#[derive(Deserialize)]
pub struct PoolJoinForm {
    board: String,
}

pub async fn pool_join_handler(
    State(state): State<Arc<AppState>>,
    Form(form): Form<PoolJoinForm>,
) -> impl IntoResponse {
    let setup: Vec<Vec<Cell>> = match serde_json::from_str(&form.board) {
        Ok(s) => s,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid board JSON").into_response(),
    };

    if setup.len() != 4 || setup.iter().any(|row| row.len() != 10) {
        return (StatusCode::BAD_REQUEST, "Board must be 4 rows of 10").into_response();
    }

    let mut ps = state.pool_state.lock().await;

    // Remove stale entries whose poll_id hasn't been polled in 10s
    let stale_threshold = std::time::Duration::from_secs(10);
    ps.entries.retain(|entry| {
        if let Some(last_polled) = state.pool_last_polled.get(&entry.poll_id) {
            last_polled.elapsed() < stale_threshold
        } else {
            entry.created_at.elapsed() < stale_threshold
        }
    });

    if let Some(oldest) = ps.entries.first().cloned() {
        // Tentative match — don't create game yet.
        // Both players must confirm via one more poll before we finalize.
        ps.entries.remove(0);

        let blue_poll_id = uuid::Uuid::new_v4().simple().to_string()[..8].to_string();

        ps.pending.push(PendingMatch {
            red_setup: oldest.setup,
            blue_setup: setup,
            red_poll_id: oldest.poll_id,
            blue_poll_id: blue_poll_id.clone(),
            red_confirmed: false,
            blue_confirmed: false,
            created_at: Instant::now(),
        });

        // Joiner also polls — they'll discover the match via pool_status
        return Json(serde_json::json!({"matched": false, "poll_id": blue_poll_id}))
            .into_response();
    }

    // Empty pool — become the host
    let poll_id = uuid::Uuid::new_v4().simple().to_string()[..8].to_string();
    ps.entries.push(PoolEntry {
        setup,
        poll_id: poll_id.clone(),
        created_at: Instant::now(),
    });

    Json(serde_json::json!({"matched": false, "poll_id": poll_id})).into_response()
}

// ----- Pool Status (GET) -----

#[derive(Deserialize)]
pub struct PoolStatusQuery {
    poll_id: String,
}

pub async fn pool_status_handler(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PoolStatusQuery>,
) -> impl IntoResponse {
    // Record that this poll_id is still alive
    state
        .pool_last_polled
        .insert(query.poll_id.clone(), Instant::now());

    // Check pending matches for this poll_id
    let finalize = {
        let mut ps = state.pool_state.lock().await;

        // Clean up expired pending matches (5s without both confirming)
        let confirm_timeout = std::time::Duration::from_secs(5);
        let mut requeue = vec![];
        ps.pending.retain(|pm| {
            if pm.created_at.elapsed() < confirm_timeout {
                return true; // still fresh
            }
            // Expired — re-queue whichever side confirmed
            if pm.red_confirmed && !pm.blue_confirmed {
                requeue.push(PoolEntry {
                    setup: pm.red_setup.clone(),
                    poll_id: pm.red_poll_id.clone(),
                    created_at: Instant::now(),
                });
            } else if pm.blue_confirmed && !pm.red_confirmed {
                requeue.push(PoolEntry {
                    setup: pm.blue_setup.clone(),
                    poll_id: pm.blue_poll_id.clone(),
                    created_at: Instant::now(),
                });
            }
            false // remove expired
        });
        ps.entries.extend(requeue);

        // Find and confirm this poll_id
        let mut result = None;
        ps.pending.retain(|pm| {
            if result.is_some() {
                return true;
            }
            let is_red = pm.red_poll_id == query.poll_id;
            let is_blue = pm.blue_poll_id == query.poll_id;
            if !is_red && !is_blue {
                return true; // not ours
            }
            let both = (is_red && pm.blue_confirmed) || (is_blue && pm.red_confirmed);
            if both {
                // Both confirmed — extract for game creation
                result = Some(pm.clone());
                false // remove from pending
            } else {
                true // keep, mark confirmed on next mutable pass
            }
        });

        // If not yet both confirmed, mark our side
        if result.is_none() {
            for pm in &mut ps.pending {
                if pm.red_poll_id == query.poll_id {
                    pm.red_confirmed = true;
                    break;
                }
                if pm.blue_poll_id == query.poll_id {
                    pm.blue_confirmed = true;
                    break;
                }
            }
        }

        result
    }; // lock dropped

    // If both confirmed, create the game
    if let Some(pm) = finalize {
        let mut red_setup = pm.red_setup;
        for row in &mut red_setup {
            for cell in row {
                if let Cell::Piece(p) = cell {
                    p.side = 0;
                }
            }
        }

        let mut game = Game {
            id: uuid::Uuid::new_v4().to_string(),
            red_hash: uuid::Uuid::new_v4().hex_string_short(),
            blue_hash: uuid::Uuid::new_v4().hex_string_short(),
            join_hash: uuid::Uuid::new_v4().hex_string_short(),
            board: Game::empty_board(),
            red_setup: None,
            blue_setup: None,
            moves: vec![],
            turn: false,
            private: false,
            game_state: 0,
            created: chrono::Utc::now().to_rfc3339(),
            modified: chrono::Utc::now().to_rfc3339(),
        };
        game.set_red_setup(&red_setup);
        game.set_blocks();

        let mut blue_setup = pm.blue_setup;
        for row in &mut blue_setup {
            for cell in row {
                if let Cell::Piece(p) = cell {
                    p.side = 1;
                }
            }
        }
        game.set_blue_setup(&blue_setup);
        game.game_state = 1; // READY

        if let Err(e) = state.storage.save_game(&game).await {
            tracing::error!("Failed to save pool game: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }

        // Store results for both players to discover via their next poll
        state
            .pool_matches
            .insert(pm.red_poll_id, game.red_hash);
        state
            .pool_matches
            .insert(pm.blue_poll_id, game.blue_hash);
    }

    // Check if this poll_id has a finalized match
    if let Some((_, player_hash)) = state.pool_matches.remove(&query.poll_id) {
        return Json(serde_json::json!({
            "matched": true,
            "player_hash": player_hash
        }))
        .into_response();
    }

    Json(serde_json::json!({"matched": false})).into_response()
}

// ----- Pool Leave -----

#[derive(Deserialize)]
pub struct PoolLeaveForm {
    poll_id: String,
}

pub async fn pool_leave_handler(
    State(state): State<Arc<AppState>>,
    Form(form): Form<PoolLeaveForm>,
) -> impl IntoResponse {
    let mut ps = state.pool_state.lock().await;
    ps.entries.retain(|entry| entry.poll_id != form.poll_id);
    ps.pending.retain(|pm| {
        pm.red_poll_id != form.poll_id && pm.blue_poll_id != form.poll_id
    });
    state.pool_last_polled.remove(&form.poll_id);
    StatusCode::OK
}

#[derive(Clone)]
pub struct PoolEntry {
    pub setup: Vec<Vec<Cell>>,
    pub poll_id: String,
    pub created_at: Instant,
}

#[derive(Clone)]
pub struct PendingMatch {
    pub red_setup: Vec<Vec<Cell>>,
    pub blue_setup: Vec<Vec<Cell>>,
    pub red_poll_id: String,
    pub blue_poll_id: String,
    pub red_confirmed: bool,
    pub blue_confirmed: bool,
    pub created_at: Instant,
}

pub struct PoolState {
    pub entries: Vec<PoolEntry>,
    pub pending: Vec<PendingMatch>,
}

// Helper trait to generate short hex strings from UUIDs
trait UuidExt {
    fn hex_string_short(&self) -> String;
}

impl UuidExt for uuid::Uuid {
    fn hex_string_short(&self) -> String {
        self.simple().to_string()[..6].to_string()
    }
}
