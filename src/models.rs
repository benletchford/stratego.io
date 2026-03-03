use serde::{Deserialize, Serialize};

/// A cell on the board: either empty (0), water (1), or occupied by a piece.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum Cell {
    Empty(i32),
    Piece(Piece),
}

impl Cell {
    pub fn is_empty(&self) -> bool {
        matches!(self, Cell::Empty(0))
    }

    pub fn is_water(&self) -> bool {
        matches!(self, Cell::Empty(1))
    }

    pub fn is_occupied(&self) -> bool {
        matches!(self, Cell::Piece(_))
    }

    pub fn as_piece(&self) -> Option<&Piece> {
        match self {
            Cell::Piece(p) => Some(p),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Piece {
    pub rank: String,
    pub side: i32,
}

impl Piece {
    pub fn new(rank: &str, side: i32) -> Self {
        Self {
            rank: rank.to_string(),
            side,
        }
    }

    pub fn unknown(opponent_side: i32) -> Self {
        Self {
            rank: "U".to_string(),
            side: opponent_side,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub type Board = Vec<Vec<Cell>>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Game {
    pub id: String,
    pub red_hash: String,
    pub blue_hash: String,
    pub join_hash: String,
    pub board: Board,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub red_setup: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_setup: Option<serde_json::Value>,
    pub moves: Vec<serde_json::Value>,
    pub turn: bool,
    pub private: bool,
    pub game_state: i32,
    pub created: String,
    pub modified: String,
}

impl Game {
    pub fn empty_board() -> Board {
        vec![vec![Cell::Empty(0); 10]; 10]
    }

    pub fn get_piece(&self, pos: &Position) -> &Cell {
        &self.board[pos.y][pos.x]
    }

    pub fn set_piece(&mut self, pos: &Position, cell: Cell) {
        self.board[pos.y][pos.x] = cell;
    }

    pub fn move_piece(&mut self, from: &Position, to: &Position) {
        let piece = self.board[from.y][from.x].clone();
        self.board[from.y][from.x] = Cell::Empty(0);
        self.board[to.y][to.x] = piece;
    }

    pub fn delete_piece(&mut self, pos: &Position) {
        self.board[pos.y][pos.x] = Cell::Empty(0);
    }

    pub fn flip_turn(&mut self) {
        self.turn = !self.turn;
    }

    pub fn get_last_move(&self) -> Option<serde_json::Value> {
        self.moves.last().cloned()
    }

    pub fn set_last_move(&mut self, last_move: serde_json::Value) {
        self.moves.push(last_move);
    }

    pub fn has_ended(&self) -> bool {
        if let Some(last_move) = self.get_last_move() {
            last_move.get("type").and_then(|t| t.as_str()) == Some("capture")
        } else {
            false
        }
    }

    pub fn get_opponent_hash(&self, player_hash: &str) -> Option<&str> {
        if player_hash == self.blue_hash {
            Some(&self.red_hash)
        } else if player_hash == self.red_hash {
            Some(&self.blue_hash)
        } else {
            None
        }
    }

    /// Set red's initial piece setup into rows 6-9
    pub fn set_red_setup(&mut self, setup: &[Vec<Cell>]) {
        self.board[6] = setup[0].clone();
        self.board[7] = setup[1].clone();
        self.board[8] = setup[2].clone();
        self.board[9] = setup[3].clone();
        self.red_setup = Some(serde_json::to_value(setup).unwrap());
    }

    /// Set blue's initial piece setup into rows 0-3 (reversed)
    pub fn set_blue_setup(&mut self, setup: &[Vec<Cell>]) {
        let mut row0 = setup[0].clone();
        row0.reverse();
        let mut row1 = setup[1].clone();
        row1.reverse();
        let mut row2 = setup[2].clone();
        row2.reverse();
        let mut row3 = setup[3].clone();
        row3.reverse();

        self.board[3] = row0;
        self.board[2] = row1;
        self.board[1] = row2;
        self.board[0] = row3;
        self.blue_setup = Some(serde_json::to_value(setup).unwrap());
    }

    /// Place water blocks on the board
    pub fn set_blocks(&mut self) {
        use crate::constants::WATER_POSITIONS;
        for &(x, y) in &WATER_POSITIONS {
            self.board[y][x] = Cell::Empty(1);
        }
    }

    /// Reverse the entire board 180 degrees
    pub fn reverse_board(board: &Board) -> Board {
        let mut reversed: Board = board.iter().rev().cloned().collect();
        for row in &mut reversed {
            row.reverse();
        }
        reversed
    }
}
