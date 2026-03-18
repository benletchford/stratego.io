use std::cell::RefCell;
use std::rc::Rc;

pub(crate) async fn sleep_ms(ms: u32) {
    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        let win = web_sys::window().unwrap();
        let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms as i32);
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

/// Polls `GET /api/game?player_hash=...` at a fixed interval.
/// Calls `on_update` whenever the game's `modified` timestamp changes.
/// The returned `Rc` keeps the poller alive; drop it to stop polling.
pub struct Poller {
    active: Rc<RefCell<bool>>,
}

impl Poller {
    pub fn start(
        player_hash: String,
        interval_ms: u32,
        on_update: impl Fn(serde_json::Value) + 'static,
    ) -> Self {
        let active = Rc::new(RefCell::new(true));
        let active_clone = active.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let last_modified: Rc<RefCell<String>> = Rc::new(RefCell::new(String::new()));

            loop {
                if !*active_clone.borrow() {
                    break;
                }

                // Poll the game endpoint
                match crate::api::get_game(&player_hash).await {
                    Ok(game) => {
                        let modified = game
                            .get("modified")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();

                        let changed = {
                            let last = last_modified.borrow();
                            *last != modified
                        };

                        if changed {
                            *last_modified.borrow_mut() = modified;
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

        Self { active }
    }

    pub fn stop(&self) {
        *self.active.borrow_mut() = false;
    }
}

impl Drop for Poller {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Polls `GET /api/pool/status?poll_id=...` to check for a pool match.
/// Times out after `timeout_ms` and calls `on_timeout` if no match is found.
pub struct PoolPoller {
    active: Rc<RefCell<bool>>,
}

impl PoolPoller {
    pub fn start(
        poll_id: String,
        interval_ms: u32,
        timeout_ms: u32,
        on_matched: impl Fn(String) + 'static,
        on_timeout: impl Fn() + 'static,
    ) -> Self {
        let active = Rc::new(RefCell::new(true));
        let active_clone = active.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let mut elapsed: u32 = 0;
            loop {
                if !*active_clone.borrow() {
                    break;
                }

                if elapsed >= timeout_ms {
                    on_timeout();
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
                elapsed += interval_ms;
            }
        });

        Self { active }
    }

    pub fn stop(&self) {
        *self.active.borrow_mut() = false;
    }
}

impl Drop for PoolPoller {
    fn drop(&mut self) {
        self.stop();
    }
}
