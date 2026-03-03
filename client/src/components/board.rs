use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::window;

const MIN_WIDTH: f64 = 320.0;

/// Board container with responsive sizing.
/// Matches the original BoardView.coffee + board.jade structure.
///
/// HTML: <div class="board-view image-board-no-trees" style="...">
///         <div class="content-container">
///           {children}
///         </div>
///       </div>
#[component]
pub fn Board(children: Children) -> impl IntoView {
    let (style, set_style) = signal(String::new());

    let resize = move || {
        let win = window().unwrap();
        let w = win.inner_width().unwrap().as_f64().unwrap();
        let h = win.inner_height().unwrap().as_f64().unwrap();
        let min = w.min(h).max(MIN_WIDTH);

        let css = if w > h {
            format!(
                "left: 50%; margin-left: {}px; top: 0; margin-top: 0; \
                 border-left: 1px solid rgba(0,0,0,0.2); border-right: 1px solid rgba(0,0,0,0.2); \
                 border-top: 0; border-bottom: 0; \
                 width: {}px; height: {}px;",
                -(min / 2.0) as i32,
                min as i32,
                min as i32
            )
        } else {
            format!(
                "top: 50%; margin-top: {}px; left: 0; margin-left: 0; \
                 border-top: 1px solid rgba(0,0,0,0.2); border-bottom: 1px solid rgba(0,0,0,0.2); \
                 border-left: 0; border-right: 0; \
                 width: {}px; height: {}px;",
                -(min / 2.0) as i32,
                min as i32,
                min as i32
            )
        };

        set_style.set(css);
    };

    // Resize on mount
    resize();

    // Resize on window resize
    let handler = Closure::<dyn Fn()>::new(move || {
        resize();
    });
    let win = window().unwrap();
    let _ = win.add_event_listener_with_callback("resize", handler.as_ref().unchecked_ref());
    handler.forget();

    view! {
        <div class="board-view image-board-no-trees" style=style>
            <div class="content-container">
                {children()}
            </div>
        </div>
    }
}
