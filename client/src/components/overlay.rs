use leptos::prelude::*;
use web_sys::window;

struct ImageDef {
    base_width: f64,
    base_height: f64,
}

const TREES: &[(&str, ImageDef)] = &[
    ("tree1", ImageDef { base_width: 49.0, base_height: 54.0 }),
    ("tree2", ImageDef { base_width: 49.0, base_height: 53.0 }),
    ("tree3", ImageDef { base_width: 49.0, base_height: 53.0 }),
];

const GRASSES: &[(&str, ImageDef)] = &[
    ("grass1", ImageDef { base_width: 24.0, base_height: 20.0 }),
    ("grass2", ImageDef { base_width: 16.0, base_height: 11.0 }),
];

const PADDING: f64 = 20.0;

fn random() -> f64 {
    js_sys::Math::random()
}

struct Rect {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
}

#[derive(Clone)]
struct OverlayItem {
    class_name: String,
    width: f64,
    height: f64,
    margin_left: f64,
    margin_top: f64,
    rotation: i32,
}

fn create_item(rect: &Rect, kind: &str) -> OverlayItem {
    let images = if kind == "tree" { TREES } else { GRASSES };
    let idx = (random() * images.len() as f64) as usize;
    let (name, def) = &images[idx.min(images.len() - 1)];

    let width = def.base_width;
    let height = def.base_height;

    let (margin_left, margin_top) = if rect.bottom > rect.right {
        let top = if rect.top != 0.0 {
            let range = rect.bottom - rect.top - height - PADDING;
            range * random() + rect.top
        } else {
            (rect.bottom - height) * random()
        };
        let left = (rect.right - width - PADDING) * random();
        (left, top)
    } else {
        let left = if rect.left != 0.0 {
            let range = rect.right - rect.left - width - PADDING;
            range * random() + rect.left
        } else {
            (rect.right - width) * random()
        };
        let top = (rect.bottom - height - PADDING) * random();
        (left, top)
    };

    let rotation = (360.0 * random()) as i32;

    OverlayItem {
        class_name: format!("overlay-graphic-view image-{}", name),
        width,
        height,
        margin_left,
        margin_top,
        rotation,
    }
}

/// Decorative grass and trees placed around the board.
/// Matches the original OverlayGraphicView.coffee / OverlayGraphicsView.coffee.
#[component]
pub fn OverlayGraphics() -> impl IntoView {
    let (items, set_items) = signal(Vec::<OverlayItem>::new());

    let generate = move || {
        let win = window().unwrap();
        let w = win.inner_width().unwrap().as_f64().unwrap();
        let h = win.inner_height().unwrap().as_f64().unwrap();

        let min = w.min(h).max(320.0);
        let max = w.max(h);

        // Calculate board offset (mirrors Board component logic)
        let (board_left, board_top) = if w > h {
            ((w - min) / 2.0, 0.0)
        } else {
            (0.0, (h - min) / 2.0)
        };

        let (rect1, rect2) = if w > h {
            (
                Rect { left: 0.0, right: board_left, top: 0.0, bottom: h },
                Rect { left: board_left + min, right: w, top: 0.0, bottom: h },
            )
        } else {
            (
                Rect { left: 0.0, right: w, top: 0.0, bottom: board_top },
                Rect { left: 0.0, right: w, top: board_top + min, bottom: h },
            )
        };

        let amount = ((max - min) / 100.0).round() as usize;
        let mut new_items = Vec::new();

        if amount > 1 {
            for _ in 0..=amount {
                new_items.push(create_item(&rect1, "grass"));
                new_items.push(create_item(&rect2, "grass"));
            }
            for _ in 0..=amount {
                new_items.push(create_item(&rect1, "tree"));
                new_items.push(create_item(&rect2, "tree"));
            }
        }

        set_items.set(new_items);
    };

    // Generate on mount
    generate();

    // Regenerate on window resize
    let handler = wasm_bindgen::closure::Closure::<dyn Fn()>::new(move || {
        generate();
    });
    let win = window().unwrap();
    let _ = win.add_event_listener_with_callback("resize", handler.as_ref().unchecked_ref());
    handler.forget();

    view! {
        {move || items.get().into_iter().map(|item| {
            let style = format!(
                "width: {}px; height: {}px; margin-left: {}px; margin-top: {}px; transform: rotate({}deg);",
                item.width, item.height, item.margin_left, item.margin_top, item.rotation
            );
            view! {
                <div class=item.class_name style=style></div>
            }
        }).collect::<Vec<_>>()}
    }
}

use wasm_bindgen::JsCast;
