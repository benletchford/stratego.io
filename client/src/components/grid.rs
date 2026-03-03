use leptos::prelude::*;
use stratego::models::{Board, Cell, Piece};
use web_sys::DragEvent;

use super::piece::PieceView;

/// Last move info for rendering attack results and highlighting.
#[derive(Clone, Debug, PartialEq)]
pub struct LastMoveInfo {
    pub from_x: usize,
    pub from_y: usize,
    pub to_x: usize,
    pub to_y: usize,
    pub move_type: String,
    pub from_piece: Option<Piece>,
    pub to_piece: Option<Piece>,
}

/// Move event: (from_x, from_y, to_x, to_y)
pub type MoveEvent = (usize, usize, usize, usize);

/// 10x10 game grid with drag-drop and click-to-select support.
/// Matches the original GridView.coffee + grid.jade structure.
#[component]
pub fn Grid(
    board: Signal<Board>,
    side: Signal<i32>,
    last_move: Signal<Option<LastMoveInfo>>,
    set_move: WriteSignal<Option<MoveEvent>>,
) -> impl IntoView {
    // Track which cell is currently selected (click-to-move)
    let (selected, set_selected) = signal(Option::<(usize, usize)>::None);
    // Track which cell has hover
    let (hover, set_hover) = signal(Option::<(usize, usize)>::None);

    view! {
        <div class="grid-view">
            {move || {
                let board_val = board.get();
                let side_val = side.get();
                let last = last_move.get();
                let sel = selected.get();
                let hov = hover.get();

                board_val.iter().enumerate().flat_map(|(y, row)| {
                    row.iter().enumerate().map(|(x, cell)| {
                        let cell = cell.clone();

                        // Determine CSS classes for this cell
                        let mut classes = vec!["cell".to_string()];

                        if let Some((sx, sy)) = sel {
                            if sx == x && sy == y {
                                classes.push("selected".to_string());
                            }
                        }

                        if let Some((hx, hy)) = hov {
                            if hx == x && hy == y {
                                classes.push("hover".to_string());
                            }
                        }

                        // Last move highlighting
                        let mut last_from = false;
                        let mut last_to = false;
                        if let Some(ref lm) = last {
                            if lm.from_x == x && lm.from_y == y {
                                classes.push("last-move-from".to_string());
                                last_from = true;
                            }
                            if lm.to_x == x && lm.to_y == y {
                                classes.push("last-move-to".to_string());
                                last_to = true;
                            }
                        }

                        let class_str = classes.join(" ");

                        // Figure out what to render inside the cell
                        let cell_content = if last_from {
                            if let Some(ref lm) = last {
                                match lm.move_type.as_str() {
                                    "draw" | "lost" => {
                                        lm.from_piece.clone().map(|p| view! {
                                            <PieceView piece=p phantom=true />
                                        }.into_any())
                                    }
                                    _ => render_cell_piece(&cell),
                                }
                            } else {
                                render_cell_piece(&cell)
                            }
                        } else if last_to {
                            if let Some(ref lm) = last {
                                match lm.move_type.as_str() {
                                    "draw" => {
                                        lm.to_piece.clone().map(|p| view! {
                                            <PieceView piece=p phantom=true />
                                        }.into_any())
                                    }
                                    "lost" => {
                                        lm.to_piece.clone().map(|p| view! {
                                            <PieceView piece=p />
                                        }.into_any())
                                    }
                                    "won" | "capture" => {
                                        match (&lm.from_piece, &lm.to_piece) {
                                            (Some(fp), Some(tp)) => Some(view! {
                                                <PieceView piece=fp.clone() dead_piece=tp.clone() />
                                            }.into_any()),
                                            _ => render_cell_piece(&cell),
                                        }
                                    }
                                    _ => render_cell_piece(&cell),
                                }
                            } else {
                                render_cell_piece(&cell)
                            }
                        } else {
                            render_cell_piece(&cell)
                        };

                        // Can this piece be dragged?
                        let draggable = matches!(&cell, Cell::Piece(p) if p.side == side_val || p.side == 3);
                        let draggable_str = if draggable { "true" } else { "false" };

                        // Can this cell be selected via click?
                        let can_select = matches!(&cell, Cell::Piece(p) if p.rank != "U");
                        let is_own_piece = matches!(&cell, Cell::Piece(p) if p.side == side_val || p.side == 3);

                        // Event handlers
                        let on_dragstart = move |e: DragEvent| {
                            set_selected.set(Some((x, y)));
                            if let Some(dt) = e.data_transfer() {
                                let data = format!("{{\"from\":{{\"x\":{},\"y\":{}}}}}", x, y);
                                let _ = dt.set_data("text/plain", &data);
                            }
                        };

                        let on_dragover = move |e: DragEvent| {
                            e.prevent_default();
                            set_hover.set(Some((x, y)));
                        };

                        let on_dragleave = move |_e: DragEvent| {
                            set_hover.update(|h| {
                                if *h == Some((x, y)) { *h = None; }
                            });
                        };

                        let on_drop = move |e: DragEvent| {
                            e.prevent_default();
                            set_hover.set(None);
                            set_selected.set(None);
                            if let Some(dt) = e.data_transfer() {
                                if let Ok(data) = dt.get_data("text/plain") {
                                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&data) {
                                        if let (Some(fx), Some(fy)) = (
                                            parsed.get("from").and_then(|f| f.get("x")).and_then(|v| v.as_u64()),
                                            parsed.get("from").and_then(|f| f.get("y")).and_then(|v| v.as_u64()),
                                        ) {
                                            set_move.set(Some((fx as usize, fy as usize, x, y)));
                                        }
                                    }
                                }
                            }
                        };

                        let on_click = move |_e: web_sys::MouseEvent| {
                            if let Some((fx, fy)) = selected.get_untracked() {
                                set_selected.set(None);
                                set_hover.set(None);
                                set_move.set(Some((fx, fy, x, y)));
                            } else if is_own_piece && can_select {
                                set_selected.set(Some((x, y)));
                            }
                        };

                        let on_mouseover = move |_e: web_sys::MouseEvent| {
                            if (is_own_piece && can_select) || selected.get_untracked().is_some() {
                                set_hover.set(Some((x, y)));
                            }
                        };

                        let on_mouseleave = move |_e: web_sys::MouseEvent| {
                            set_hover.update(|h| {
                                if *h == Some((x, y)) { *h = None; }
                            });
                        };

                        view! {
                            <div
                                class=class_str
                                data-x=x.to_string()
                                data-y=y.to_string()
                                draggable=draggable_str
                                on:dragstart=on_dragstart
                                on:dragover=on_dragover
                                on:dragleave=on_dragleave
                                on:drop=on_drop
                                on:click=on_click
                                on:mouseover=on_mouseover
                                on:mouseleave=on_mouseleave
                            >
                                {cell_content}
                            </div>
                        }
                    }).collect::<Vec<_>>()
                }).collect::<Vec<_>>()
            }}
        </div>
    }
}

fn render_cell_piece(cell: &Cell) -> Option<AnyView> {
    match cell {
        Cell::Piece(p) => Some(
            view! { <PieceView piece=p.clone() /> }.into_any()
        ),
        _ => None,
    }
}
