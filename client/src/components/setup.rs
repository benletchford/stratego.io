use leptos::prelude::*;
use stratego::constants::RANKS;
use stratego::models::{Board, Cell, Piece};

use super::grid::{Grid, LastMoveInfo, MoveEvent};
use super::loading::Loading;

/// Default 4x10 board setup using RANKS from shared crate.
fn default_setup() -> Board {
    let mut pieces = Vec::new();
    for &(rank, amount) in RANKS {
        for _ in 0..amount {
            pieces.push(Cell::Piece(Piece::new(rank, 3)));
        }
    }

    let mut board = Vec::new();
    for _ in 0..4 {
        let row: Vec<Cell> = pieces.drain(..10).collect();
        board.push(row);
    }
    board
}

/// Try to load board from localStorage, fall back to default.
fn load_saved_setup() -> Board {
    let win = web_sys::window().unwrap();
    if let Ok(Some(storage)) = win.local_storage() {
        if let Ok(Some(json)) = storage.get_item("lastBoard") {
            if let Ok(board) = serde_json::from_str::<Board>(&json) {
                if board.len() == 4 && board.iter().all(|r| r.len() == 10) {
                    return board;
                }
            }
        }
    }
    default_setup()
}

/// Save board to localStorage.
fn save_setup(board: &Board) {
    let win = web_sys::window().unwrap();
    if let Ok(Some(storage)) = win.local_storage() {
        if let Ok(json) = serde_json::to_string(board) {
            let _ = storage.set_item("lastBoard", &json);
        }
    }
}

/// Setup page for arranging pieces before starting a game.
/// Matches the original SetupView.coffee + setup.jade structure.
#[component]
pub fn SetupPage(
    #[prop(into)] mode: String,
    #[prop(into)] hash: String,
) -> impl IntoView {
    let initial_board = load_saved_setup();
    let (board, set_board) = signal(initial_board);
    let side = Signal::derive(|| 3_i32);
    let last_move = Signal::derive(|| Option::<LastMoveInfo>::None);

    // Move signal: Grid writes swap events here
    let (move_event, set_move_event) = signal(Option::<MoveEvent>::None);

    // Loading state: None = setup visible, Some(msg) = loading screen
    let (loading_msg, set_loading_msg) = signal(Option::<String>::None);

    // Watch for swap events from the grid
    Effect::new(move |_| {
        if let Some((fx, fy, tx, ty)) = move_event.get() {
            set_board.update(|b| {
                if fy < b.len() && fx < 10 && ty < b.len() && tx < 10 {
                    let from_piece = b[fy][fx].clone();
                    let to_piece = b[ty][tx].clone();
                    b[fy][fx] = to_piece;
                    b[ty][tx] = from_piece;
                }
            });
            set_move_event.set(None);
        }
    });

    let mode = StoredValue::new(mode);
    let hash = StoredValue::new(hash);

    // Start handler
    let navigate = leptos_router::hooks::use_navigate();
    let on_start = move |_: web_sys::MouseEvent| {
        let current_board = board.get_untracked();
        save_setup(&current_board);

        let board_json = serde_json::to_string(&current_board).unwrap();
        let mode_val = mode.get_value();
        let hash_val = hash.get_value();
        let nav = navigate.clone();

        let msg = match mode_val.as_str() {
            "create" => "Creating game...",
            "join" => "Joining game...",
            _ => "Starting...",
        };
        set_loading_msg.set(Some(msg.to_string()));

        wasm_bindgen_futures::spawn_local(async move {
            match mode_val.as_str() {
                "create" => {
                    match crate::api::create_game(&board_json).await {
                        Ok(game) => {
                            if let Some(ph) = game.get("player_hash").and_then(|v| v.as_str()) {
                                nav(&format!("/play/{}", ph), Default::default());
                            }
                        }
                        Err(e) => {
                            web_sys::console::error_1(&format!("Create error: {}", e).into());
                            set_loading_msg.set(None);
                        }
                    }
                }
                "join" => {
                    match crate::api::join_game(&hash_val, &board_json).await {
                        Ok(game) => {
                            if let Some(ph) = game.get("player_hash").and_then(|v| v.as_str()) {
                                nav(&format!("/play/{}", ph), Default::default());
                            }
                        }
                        Err(e) => {
                            web_sys::console::error_1(&format!("Join error: {}", e).into());
                            set_loading_msg.set(None);
                        }
                    }
                }
                "pool" => {
                    nav("/play/pool", Default::default());
                }
                _ => {}
            }
        });
    };

    let loading_signal = Signal::derive(move || {
        loading_msg.get().unwrap_or_default()
    });

    let is_loading = move || loading_msg.get().is_some();

    view! {
        <div style=move || if is_loading() { "" } else { "display:none" }>
            <Loading message=loading_signal />
        </div>
        <div class="setup-view" style=move || if is_loading() { "display:none" } else { "" }>
            <div class="panel">
                    <a class="panel-link-view panel-option" href="/">
                        <div class="title">"Back"</div>
                        <div class="description">"Go back to the main menu."</div>
                    </a>
                    <button class="panel-button-view panel-option" on:click=on_start>
                        <div class="title">"Start"</div>
                        <div class="description">"Once you're happy with the setup click here to start the game."</div>
                    </button>
                    <div class="panel-textbox-view panel-textbox">
                        "This is where you setup your pieces to better protect your flag."
                        <br /><br />
                        "Click or drag to rearrange. Your last used setup will be saved."
                    </div>
                </div>
                <div class="grid-container">
                    {(0..3).map(|i| view! {
                        <div class="horizontal-grid-line" data-number=i.to_string()></div>
                    }).collect::<Vec<_>>()}
                    {(0..9).map(|i| view! {
                        <div class="vertical-grid-line" data-number=i.to_string()></div>
                    }).collect::<Vec<_>>()}

                    <Grid
                        board=board.into()
                        side=side
                        last_move=last_move
                        set_move=set_move_event
                    />
                </div>
        </div>
    }
}
