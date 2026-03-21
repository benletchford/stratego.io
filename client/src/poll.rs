use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub(crate) async fn sleep_ms(ms: u32) {
    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        let win = web_sys::window().unwrap();
        let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms as i32);
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

/// A Send+Sync handle that can stop a poller from outside (e.g. in on_cleanup).
/// Create this *before* spawning async work so cleanup is always registered.
#[derive(Clone)]
pub struct StopHandle(Arc<AtomicBool>);

impl StopHandle {
    pub fn new() -> Self {
        Self(Arc::new(AtomicBool::new(true)))
    }

    pub fn stop(&self) {
        self.0.store(false, Ordering::Relaxed);
    }

    fn is_active(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

/// Polls `GET /api/game?player_hash=...` at a fixed interval.
/// Calls `on_update` whenever the game's `modified` timestamp changes.
pub fn start_game_poller(
    handle: StopHandle,
    player_hash: String,
    interval_ms: u32,
    on_update: impl Fn(serde_json::Value) + 'static,
) {
    wasm_bindgen_futures::spawn_local(async move {
        let mut last_modified = String::new();

        loop {
            if !handle.is_active() {
                break;
            }

            match crate::api::get_game(&player_hash).await {
                Ok(game) => {
                    let modified = game
                        .get("modified")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();

                    if last_modified != modified {
                        last_modified = modified;
                        on_update(game);
                    }
                }
                Err(e) => {
                    web_sys::console::error_1(
                        &format!("Poll error: {}", e).into(),
                    );
                }
            }

            sleep_ms(interval_ms).await;
        }
    });
}

/// Polls `GET /api/pool/status?poll_id=...` until a match is found.
pub fn start_pool_poller(
    handle: StopHandle,
    poll_id: String,
    interval_ms: u32,
    on_matched: impl Fn(String) + 'static,
) {
    wasm_bindgen_futures::spawn_local(async move {
        loop {
            if !handle.is_active() {
                break;
            }

            match crate::api::poll_pool(&poll_id).await {
                Ok(Some(player_hash)) => {
                    on_matched(player_hash);
                    break;
                }
                Ok(None) => {}
                Err(e) => {
                    web_sys::console::error_1(
                        &format!("Pool poll error: {}", e).into(),
                    );
                }
            }

            sleep_ms(interval_ms).await;
        }
    });
}
