#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveType {
    Move = 0,
    AttackDraw = 1,
    AttackWon = 2,
    AttackLost = 3,
    Capture = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    WaitingForOpponent = 0,
    Ready = 1,
}

pub const WATER_POSITIONS: [(usize, usize); 8] = [
    (2, 4),
    (2, 5),
    (3, 4),
    (3, 5),
    (6, 4),
    (6, 5),
    (7, 4),
    (7, 5),
];

/// (rank, amount) for each piece type
pub const RANKS: &[(&str, u32)] = &[
    ("1", 1),
    ("2", 1),
    ("3", 2),
    ("4", 3),
    ("5", 4),
    ("6", 4),
    ("7", 4),
    ("8", 5),
    ("9", 8),
    ("S", 1),
    ("B", 6),
    ("F", 1),
];
