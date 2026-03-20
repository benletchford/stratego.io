use web_sys::{Notification, NotificationOptions, NotificationPermission};
use wasm_bindgen::JsCast;

/// Request notification permission (no-op if already granted/denied).
pub fn request_permission() {
    if !is_supported() {
        return;
    }
    if Notification::permission() == NotificationPermission::Default {
        let _ = Notification::request_permission();
    }
}

/// Show a notification if permission is granted.
pub fn notify(title: &str, body: &str) {
    if !is_supported() {
        return;
    }
    if Notification::permission() != NotificationPermission::Granted {
        return;
    }

    let opts = NotificationOptions::new();
    opts.set_body(body);

    if let Ok(n) = Notification::new_with_options(title, &opts) {
        // Focus the tab when clicked
        let onclick = wasm_bindgen::closure::Closure::<dyn Fn()>::new(move || {
            if let Some(win) = web_sys::window() {
                let _ = win.focus();
            }
        });
        n.set_onclick(Some(onclick.as_ref().unchecked_ref()));
        onclick.forget();
    }
}

fn is_supported() -> bool {
    js_sys::Reflect::get(&js_sys::global(), &"Notification".into())
        .map(|v| !v.is_undefined())
        .unwrap_or(false)
}
