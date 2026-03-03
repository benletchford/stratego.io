use crate::constants::MoveType;
use crate::models::{Cell, Game, Position};

#[derive(Debug)]
pub struct InvalidMove(pub String);

impl std::fmt::Display for InvalidMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for InvalidMove {}

/// Validate a move and return the move type. Port of models.py check_move.
pub fn check_move(game: &Game, from: &Position, to: &Position) -> Result<MoveType, InvalidMove> {
    let from_cell = game.get_piece(from);
    let to_cell = game.get_piece(to);

    let from_piece = match from_cell {
        Cell::Empty(_) => return Err(InvalidMove("No piece to move.".into())),
        Cell::Piece(p) => p,
    };

    // Check side matches turn: side==0 (red) when turn==false, side==1 (blue) when turn==true
    let expected_side = if game.turn { 1 } else { 0 };
    if from_piece.side != expected_side {
        return Err(InvalidMove("Not your turn".into()));
    }

    // Water blocks cannot be entered
    if matches!(to_cell, Cell::Empty(1)) {
        return Err(InvalidMove("Can not move onto an unmoveable block.".into()));
    }

    if let Cell::Piece(to_piece) = to_cell {
        if from_piece.side == to_piece.side {
            return Err(InvalidMove("Can not move onto friendly piece.".into()));
        }
    }

    if from_piece.rank == "B" {
        return Err(InvalidMove("Bombs cannot be moved.".into()));
    }
    if from_piece.rank == "F" {
        return Err(InvalidMove("Flags cannot be moved.".into()));
    }

    let diff_x = (from.x as i32 - to.x as i32).unsigned_abs() as usize;
    let diff_y = (from.y as i32 - to.y as i32).unsigned_abs() as usize;

    if diff_x == 0 && diff_y == 0 {
        return Err(InvalidMove("Position has not changed.".into()));
    }

    if will_violate_two_square_rule(game, from, to) {
        return Err(InvalidMove(
            "That move violates the two-square rule.".into(),
        ));
    }

    // Must move in exactly one axis (no diagonal)
    let single_axis = (diff_x == 0) != (diff_y == 0);
    if !single_axis {
        return Err(InvalidMove("Illegal movement.".into()));
    }

    // Normal piece: exactly 1 square. Scout: any distance in a straight line.
    let is_one_square = (diff_x == 1 && diff_y == 0) || (diff_x == 0 && diff_y == 1);
    let is_scout = from_piece.rank == "9";

    if !is_one_square && !is_scout {
        return Err(InvalidMove("Illegal movement.".into()));
    }

    // Scout: verify no pieces between from and to
    if is_scout && is_piece_between(&game.board, from, to) {
        return Err(InvalidMove("Can not jump over pieces.".into()));
    }

    if to_cell.is_occupied() {
        Ok(check_attack(from_piece, to_cell.as_piece().unwrap()))
    } else {
        Ok(MoveType::Move)
    }
}

/// Determine the outcome of an attack. Port of models.py _check_attack.
fn check_attack(
    from_piece: &crate::models::Piece,
    to_piece: &crate::models::Piece,
) -> MoveType {
    // Draw: same rank
    if from_piece.rank == to_piece.rank {
        return MoveType::AttackDraw;
    }

    // Any movable piece captures the flag
    if to_piece.rank == "F" {
        return MoveType::Capture;
    }

    // Attacking a bomb
    if to_piece.rank == "B" {
        if from_piece.rank == "8" {
            return MoveType::AttackWon; // Miner defuses bomb
        } else {
            return MoveType::AttackLost;
        }
    }

    // Everything wins against spy
    if to_piece.rank == "S" {
        return MoveType::AttackWon;
    }

    // Are we a spy?
    if from_piece.rank == "S" {
        if to_piece.rank == "1" {
            return MoveType::AttackWon; // Spy assassinates marshal
        } else {
            return MoveType::AttackLost;
        }
    }

    // Numeric comparison: lower rank number = higher rank = wins
    let from_rank: i32 = from_piece.rank.parse().unwrap();
    let to_rank: i32 = to_piece.rank.parse().unwrap();

    if to_rank > from_rank {
        MoveType::AttackWon
    } else {
        MoveType::AttackLost
    }
}

/// Check if any piece exists between from and to positions (for scout moves).
/// Port of models.py _is_piece_between.
fn is_piece_between(
    board: &crate::models::Board,
    from: &Position,
    to: &Position,
) -> bool {
    if from.y == to.y {
        // Moving on x axis
        let coefficient: i32 = if from.x < to.x { 1 } else { -1 };
        let dist = (from.x as i32 - to.x as i32).unsigned_abs() as usize;
        for i in 1..dist {
            let x = (from.x as i32 + (i as i32 * coefficient)) as usize;
            if board[from.y][x] != Cell::Empty(0) {
                return true;
            }
        }
        false
    } else {
        // Moving on y axis
        let coefficient: i32 = if from.y < to.y { 1 } else { -1 };
        let dist = (from.y as i32 - to.y as i32).unsigned_abs() as usize;
        for i in 1..dist {
            let y = (from.y as i32 + (i as i32 * coefficient)) as usize;
            if board[y][from.x] != Cell::Empty(0) {
                return true;
            }
        }
        false
    }
}

/// Check if the proposed move violates the two-square rule.
/// Port of models.py will_violate_two_square_rule.
pub fn will_violate_two_square_rule(
    game: &Game,
    from_pos: &Position,
    to_pos: &Position,
) -> bool {
    // Select all moves for the current player (even/odd indices)
    let all_moves: Vec<serde_json::Value> = game.moves.clone();
    let start = if game.turn { 1 } else { 0 };
    let player_moves: Vec<&serde_json::Value> =
        all_moves.iter().skip(start).step_by(2).collect();

    if player_moves.len() < 3 {
        return false;
    }

    let len = player_moves.len();

    // Extract positions from last 3 moves
    let extract_positions = |mv: &serde_json::Value| -> Option<(Position, Position)> {
        let from = mv.get("from")?.get("position")?;
        let to = mv.get("to")?.get("position")?;
        Some((
            Position {
                x: from.get("x")?.as_u64()? as usize,
                y: from.get("y")?.as_u64()? as usize,
            },
            Position {
                x: to.get("x")?.as_u64()? as usize,
                y: to.get("y")?.as_u64()? as usize,
            },
        ))
    };

    let move1 = match extract_positions(player_moves[len - 3]) {
        Some(m) => m,
        None => return false,
    };
    let move2 = match extract_positions(player_moves[len - 2]) {
        Some(m) => m,
        None => return false,
    };
    let move3 = match extract_positions(player_moves[len - 1]) {
        Some(m) => m,
        None => return false,
    };

    let moves_chain = [
        (&move1.0, &move1.1),
        (&move2.0, &move2.1),
        (&move3.0, &move3.1),
        (from_pos, to_pos),
    ];

    // Check if all 4 moves are the same piece (each move's from == previous move's to)
    if !check_moves_are_same_piece(&moves_chain) {
        return false;
    }

    // Get all cells traversed in each move
    let cells1 = get_cells_between_inclusive(&move1.0, &move1.1);
    let cells2 = get_cells_between_inclusive(&move2.0, &move2.1);
    let cells3 = get_cells_between_inclusive(&move3.0, &move3.1);
    let cells4 = get_cells_between_inclusive(from_pos, to_pos);

    let mut all_cells = Vec::new();
    all_cells.extend(cells1);
    all_cells.extend(cells2);
    all_cells.extend(cells3);
    all_cells.extend(cells4.iter().cloned());

    let mut duplicate_cells = 0;
    for cell in &cells4 {
        let count = all_cells.iter().filter(|c| *c == cell).count();
        if count == 4 {
            duplicate_cells += 1;
        }
    }

    duplicate_cells > 1
}

/// Check if 4 consecutive moves involve the same piece.
fn check_moves_are_same_piece(moves: &[(&Position, &Position)]) -> bool {
    if moves.len() < 4 {
        return false;
    }
    // Each move's from must equal the previous move's to
    moves[1].0 == moves[0].1 && moves[2].0 == moves[1].1 && moves[3].0 == moves[2].1
}

/// Get all cells between two positions (inclusive), moving on a single axis.
fn get_cells_between_inclusive(from: &Position, to: &Position) -> Vec<Position> {
    let mut cells = Vec::new();

    if from.x == to.x {
        // Moving on y axis
        let min_y = from.y.min(to.y);
        let max_y = from.y.max(to.y);
        for y in min_y..=max_y {
            cells.push(Position { x: from.x, y });
        }
    } else {
        // Moving on x axis
        let min_x = from.x.min(to.x);
        let max_x = from.x.max(to.x);
        for x in min_x..=max_x {
            cells.push(Position { x, y: from.y });
        }
    }

    cells
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Cell, Game, Piece};

    fn make_game() -> Game {
        let mut game = Game {
            id: "test".into(),
            red_hash: "red123".into(),
            blue_hash: "blu123".into(),
            join_hash: "join12".into(),
            board: Game::empty_board(),
            red_setup: None,
            blue_setup: None,
            moves: vec![],
            turn: false, // red's turn
            private: true,
            game_state: 1,
            created: String::new(),
            modified: String::new(),
        };
        game.set_blocks();
        game
    }

    // Use column 0 for most tests to avoid water blocks at (2,4),(3,4),(2,5),(3,5) etc.

    #[test]
    fn test_basic_move() {
        let mut game = make_game();
        game.board[7][0] = Cell::Piece(Piece::new("5", 0));
        let result = check_move(&game, &Position { x: 0, y: 7 }, &Position { x: 0, y: 6 });
        assert_eq!(result.unwrap(), MoveType::Move);
    }

    #[test]
    fn test_cannot_move_bomb() {
        let mut game = make_game();
        game.board[7][0] = Cell::Piece(Piece::new("B", 0));
        let result = check_move(&game, &Position { x: 0, y: 7 }, &Position { x: 0, y: 6 });
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().0, "Bombs cannot be moved.");
    }

    #[test]
    fn test_cannot_move_flag() {
        let mut game = make_game();
        game.board[7][0] = Cell::Piece(Piece::new("F", 0));
        let result = check_move(&game, &Position { x: 0, y: 7 }, &Position { x: 0, y: 6 });
        assert!(result.is_err());
    }

    #[test]
    fn test_attack_won() {
        let mut game = make_game();
        game.board[7][0] = Cell::Piece(Piece::new("1", 0));
        game.board[6][0] = Cell::Piece(Piece::new("5", 1));
        let result = check_move(&game, &Position { x: 0, y: 7 }, &Position { x: 0, y: 6 });
        assert_eq!(result.unwrap(), MoveType::AttackWon);
    }

    #[test]
    fn test_attack_lost() {
        let mut game = make_game();
        game.board[7][0] = Cell::Piece(Piece::new("5", 0));
        game.board[6][0] = Cell::Piece(Piece::new("1", 1));
        let result = check_move(&game, &Position { x: 0, y: 7 }, &Position { x: 0, y: 6 });
        assert_eq!(result.unwrap(), MoveType::AttackLost);
    }

    #[test]
    fn test_attack_draw() {
        let mut game = make_game();
        game.board[7][0] = Cell::Piece(Piece::new("5", 0));
        game.board[6][0] = Cell::Piece(Piece::new("5", 1));
        let result = check_move(&game, &Position { x: 0, y: 7 }, &Position { x: 0, y: 6 });
        assert_eq!(result.unwrap(), MoveType::AttackDraw);
    }

    #[test]
    fn test_capture_flag() {
        let mut game = make_game();
        game.board[7][0] = Cell::Piece(Piece::new("9", 0));
        game.board[6][0] = Cell::Piece(Piece::new("F", 1));
        let result = check_move(&game, &Position { x: 0, y: 7 }, &Position { x: 0, y: 6 });
        assert_eq!(result.unwrap(), MoveType::Capture);
    }

    #[test]
    fn test_miner_beats_bomb() {
        let mut game = make_game();
        game.board[7][0] = Cell::Piece(Piece::new("8", 0));
        game.board[6][0] = Cell::Piece(Piece::new("B", 1));
        let result = check_move(&game, &Position { x: 0, y: 7 }, &Position { x: 0, y: 6 });
        assert_eq!(result.unwrap(), MoveType::AttackWon);
    }

    #[test]
    fn test_non_miner_loses_to_bomb() {
        let mut game = make_game();
        game.board[7][0] = Cell::Piece(Piece::new("1", 0));
        game.board[6][0] = Cell::Piece(Piece::new("B", 1));
        let result = check_move(&game, &Position { x: 0, y: 7 }, &Position { x: 0, y: 6 });
        assert_eq!(result.unwrap(), MoveType::AttackLost);
    }

    #[test]
    fn test_spy_kills_marshal() {
        let mut game = make_game();
        game.board[7][0] = Cell::Piece(Piece::new("S", 0));
        game.board[6][0] = Cell::Piece(Piece::new("1", 1));
        let result = check_move(&game, &Position { x: 0, y: 7 }, &Position { x: 0, y: 6 });
        assert_eq!(result.unwrap(), MoveType::AttackWon);
    }

    #[test]
    fn test_spy_loses_to_non_marshal() {
        let mut game = make_game();
        game.board[7][0] = Cell::Piece(Piece::new("S", 0));
        game.board[6][0] = Cell::Piece(Piece::new("5", 1));
        let result = check_move(&game, &Position { x: 0, y: 7 }, &Position { x: 0, y: 6 });
        assert_eq!(result.unwrap(), MoveType::AttackLost);
    }

    #[test]
    fn test_scout_multi_square_move() {
        let mut game = make_game();
        game.board[8][0] = Cell::Piece(Piece::new("9", 0));
        let result = check_move(&game, &Position { x: 0, y: 8 }, &Position { x: 0, y: 3 });
        assert_eq!(result.unwrap(), MoveType::Move);
    }

    #[test]
    fn test_scout_blocked() {
        let mut game = make_game();
        game.board[8][0] = Cell::Piece(Piece::new("9", 0));
        game.board[5][0] = Cell::Piece(Piece::new("5", 0));
        let result = check_move(&game, &Position { x: 0, y: 8 }, &Position { x: 0, y: 3 });
        assert!(result.is_err());
    }

    #[test]
    fn test_diagonal_move_rejected() {
        let mut game = make_game();
        game.board[7][0] = Cell::Piece(Piece::new("5", 0));
        let result = check_move(&game, &Position { x: 0, y: 7 }, &Position { x: 1, y: 6 });
        assert!(result.is_err());
    }

    #[test]
    fn test_cannot_move_onto_water() {
        let mut game = make_game();
        game.board[3][2] = Cell::Piece(Piece::new("5", 0));
        // board[4][2] is water
        let result = check_move(&game, &Position { x: 2, y: 3 }, &Position { x: 2, y: 4 });
        assert!(result.is_err());
    }

    #[test]
    fn test_wrong_turn() {
        let mut game = make_game();
        game.board[7][0] = Cell::Piece(Piece::new("5", 1)); // blue piece, but red's turn
        let result = check_move(&game, &Position { x: 0, y: 7 }, &Position { x: 0, y: 6 });
        assert!(result.is_err());
    }
}
