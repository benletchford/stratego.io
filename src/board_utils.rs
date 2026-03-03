use crate::models::{Board, Cell, Game, Piece, Position};

const FLIPPED: [usize; 10] = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];

/// Reverse a position for blue perspective: (x,y) → (9-x, 9-y)
pub fn reverse_position(pos: &mut Position) {
    pos.x = FLIPPED[pos.x];
    pos.y = FLIPPED[pos.y];
}

/// Build the game dict to send to a player, hiding sensitive data.
/// Port of board_utils.py get_sendable_game.
pub fn get_sendable_game(game: &Game, side: i32) -> serde_json::Value {
    let mut dict = serde_json::json!({
        "board": get_sendable_board(game, side),
        "turn": game.turn,
        "game_state": game.game_state,
        "side": side,
        "created": game.created,
        "modified": game.modified,
    });

    // Set player_hash based on side
    if side == 0 {
        dict["player_hash"] = serde_json::json!(game.red_hash);
    } else {
        dict["player_hash"] = serde_json::json!(game.blue_hash);
    }

    // Include join_hash for the creator (red) when waiting for opponent
    if side == 0 && game.game_state == 0 {
        dict["join_hash"] = serde_json::json!(game.join_hash);
    }

    // Set last_move (with position reversal for blue)
    if !game.moves.is_empty() {
        let mut last_move = game.get_last_move().unwrap();

        if side == 1 {
            // Reverse positions for blue
            if let Some(to) = last_move.get_mut("to") {
                if let Some(pos) = to.get_mut("position") {
                    let x = pos["x"].as_u64().unwrap() as usize;
                    let y = pos["y"].as_u64().unwrap() as usize;
                    pos["x"] = serde_json::json!(FLIPPED[x]);
                    pos["y"] = serde_json::json!(FLIPPED[y]);
                }
            }
            if let Some(from) = last_move.get_mut("from") {
                if let Some(pos) = from.get_mut("position") {
                    let x = pos["x"].as_u64().unwrap() as usize;
                    let y = pos["y"].as_u64().unwrap() as usize;
                    pos["x"] = serde_json::json!(FLIPPED[x]);
                    pos["y"] = serde_json::json!(FLIPPED[y]);
                }
            }
        }

        dict["last_move"] = last_move;
    }

    dict
}

/// Build the board to send to a player, hiding opponent pieces.
/// Port of board_utils.py get_sendable_board.
pub fn get_sendable_board(game: &Game, side: i32) -> Board {
    let mut board = game.board.clone();

    // If game has ended, show full board (rotated for blue)
    if game.has_ended() {
        if side == 1 {
            return Game::reverse_board(&board);
        }
        return board;
    }

    // Red waiting for blue: hide top rows as unknown
    if side == 0 && game.blue_setup.is_none() {
        let opposite = opposite_side(side);
        board[0] = unknown_row(opposite);
        board[1] = unknown_row(opposite);
        board[2] = unknown_row(opposite);
        board[3] = unknown_row(opposite);
        return board;
    }

    // Normal: hide opponent pieces
    hide_side(&mut board, side);

    if side == 1 {
        Game::reverse_board(&board)
    } else {
        board
    }
}

/// Replace opponent pieces with unknown markers.
pub fn hide_side(board: &mut Board, side: i32) {
    let opp = opposite_side(side);
    for row in board.iter_mut() {
        for cell in row.iter_mut() {
            if let Cell::Piece(p) = cell {
                if p.side != side {
                    *cell = Cell::Piece(Piece::unknown(opp));
                }
            }
        }
    }
}

fn opposite_side(side: i32) -> i32 {
    if side == 0 {
        1
    } else {
        0
    }
}

fn unknown_row(opponent_side: i32) -> Vec<Cell> {
    vec![Cell::Piece(Piece::unknown(opponent_side)); 10]
}
