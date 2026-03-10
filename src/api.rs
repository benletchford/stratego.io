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

    let mut pool = state.pool.lock().await;

    if let Some(oldest) = pool.first().cloned() {
        // Match with the oldest waiting player
        pool.remove(0);

        // Create game with oldest as red
        let mut red_setup = oldest.setup.clone();
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

        // Join blue
        let mut blue_setup = setup;
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

        // Store match result for red player to discover via polling
        state
            .pool_matches
            .insert(oldest.poll_id.clone(), game.red_hash.clone());

        // Return match result directly to blue (the joining player)
        return Json(serde_json::json!({
            "matched": true,
            "player_hash": game.blue_hash
        }))
        .into_response();
    }

    // Empty pool - become the host, return a poll_id for checking later
    let poll_id = uuid::Uuid::new_v4().simple().to_string()[..8].to_string();
    pool.push(PoolEntry {
        setup,
        poll_id: poll_id.clone(),
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
    // Check if this poll_id has been matched
    if let Some((_, player_hash)) = state.pool_matches.remove(&query.poll_id) {
        return Json(serde_json::json!({
            "matched": true,
            "player_hash": player_hash
        }))
        .into_response();
    }

    Json(serde_json::json!({"matched": false})).into_response()
}

#[derive(Clone)]
pub struct PoolEntry {
    pub setup: Vec<Vec<Cell>>,
    pub poll_id: String,
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
