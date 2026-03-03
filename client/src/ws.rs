use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{MessageEvent, WebSocket};

type EventCallback = Box<dyn Fn(serde_json::Value)>;

struct ChannelHandlers {
    handlers: Vec<(String, EventCallback)>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ConnectionState {
    Idle,
    Connected,
    Disconnected,
    Reconnecting(u32),
}

/// Newtype wrapper for reading connection state from Leptos context.
#[derive(Clone)]
pub struct ConnectionStateSignal(pub ReadSignal<ConnectionState>);

struct WsClosures {
    on_message: Option<Closure<dyn FnMut(MessageEvent)>>,
    on_close: Option<Closure<dyn FnMut()>>,
    on_open: Option<Closure<dyn FnMut()>>,
    on_error: Option<Closure<dyn FnMut(web_sys::Event)>>,
}

pub struct WsClient {
    ws: Rc<RefCell<WebSocket>>,
    socket_id: Rc<RefCell<Option<String>>>,
    channels: Rc<RefCell<HashMap<String, ChannelHandlers>>>,
    state_writer: Rc<RefCell<Option<WriteSignal<ConnectionState>>>>,
    url: String,
    _closures: Rc<RefCell<WsClosures>>,
}

pub(crate) async fn sleep_ms(ms: u32) {
    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        let win = web_sys::window().unwrap();
        let _ = win.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms as i32);
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

#[allow(dead_code)]
impl WsClient {
    pub fn connect(state_writer: Option<WriteSignal<ConnectionState>>) -> Rc<Self> {
        let url = crate::config::ws_url();

        let ws = WebSocket::new(&url).expect("Failed to create WebSocket");

        let socket_id: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
        let channels: Rc<RefCell<HashMap<String, ChannelHandlers>>> =
            Rc::new(RefCell::new(HashMap::new()));
        let state_writer_rc = Rc::new(RefCell::new(state_writer));
        let closures = Rc::new(RefCell::new(WsClosures {
            on_message: None,
            on_close: None,
            on_open: None,
            on_error: None,
        }));

        let client = Rc::new(Self {
            ws: Rc::new(RefCell::new(ws)),
            socket_id,
            channels,
            state_writer: state_writer_rc,
            url,
            _closures: closures,
        });

        attach_handlers(
            &client.ws,
            &client.socket_id,
            &client.channels,
            &client.state_writer,
            &client.url,
            &client._closures,
        );

        client
    }

    pub fn socket_id(&self) -> Option<String> {
        self.socket_id.borrow().clone()
    }

    pub fn subscribe(&self, channel_name: &str) {
        {
            let mut channels = self.channels.borrow_mut();
            channels
                .entry(channel_name.to_string())
                .or_insert_with(|| ChannelHandlers {
                    handlers: Vec::new(),
                });
        }

        let ws = self.ws.borrow();
        if ws.ready_state() == WebSocket::OPEN {
            let msg = serde_json::json!({
                "action": "subscribe",
                "channel": channel_name
            });
            let _ = ws.send_with_str(&msg.to_string());
        }
    }

    pub fn bind(
        &self,
        channel_name: &str,
        event: &str,
        callback: impl Fn(serde_json::Value) + 'static,
    ) {
        let mut channels = self.channels.borrow_mut();
        if let Some(ch) = channels.get_mut(channel_name) {
            ch.handlers
                .push((event.to_string(), Box::new(callback)));
        }
    }

    pub fn unbind(&self, channel_name: &str, event: &str) {
        let mut channels = self.channels.borrow_mut();
        if let Some(ch) = channels.get_mut(channel_name) {
            ch.handlers.retain(|(e, _)| e != event);
        }
    }

    pub fn unsubscribe(&self, channel_name: &str) {
        self.channels.borrow_mut().remove(channel_name);
        let ws = self.ws.borrow();
        if ws.ready_state() == WebSocket::OPEN {
            let msg = serde_json::json!({
                "action": "unsubscribe",
                "channel": channel_name
            });
            let _ = ws.send_with_str(&msg.to_string());
        }
    }

    pub fn unsubscribe_all(&self, except: &[&str]) {
        let names: Vec<String> = self
            .channels
            .borrow()
            .keys()
            .filter(|k| !except.contains(&k.as_str()))
            .cloned()
            .collect();
        for name in names {
            self.unsubscribe(&name);
        }
    }

    pub fn is_open(&self) -> bool {
        self.ws.borrow().ready_state() == WebSocket::OPEN
    }
}

fn set_state(writer: &Rc<RefCell<Option<WriteSignal<ConnectionState>>>>, state: ConnectionState) {
    if let Some(w) = writer.borrow().as_ref() {
        w.set(state);
    }
}

fn attach_handlers(
    ws_rc: &Rc<RefCell<WebSocket>>,
    socket_id: &Rc<RefCell<Option<String>>>,
    channels: &Rc<RefCell<HashMap<String, ChannelHandlers>>>,
    state_writer: &Rc<RefCell<Option<WriteSignal<ConnectionState>>>>,
    url: &str,
    closures: &Rc<RefCell<WsClosures>>,
) {
    let ws = ws_rc.borrow();

    // on_open
    let sw_open = state_writer.clone();
    let on_open = Closure::wrap(Box::new(move || {
        set_state(&sw_open, ConnectionState::Connected);
    }) as Box<dyn FnMut()>);
    ws.set_onopen(Some(on_open.as_ref().unchecked_ref()));

    // on_message
    let sid_clone = socket_id.clone();
    let ch_clone = channels.clone();
    let sw_msg = state_writer.clone();
    let on_message = Closure::wrap(Box::new(move |event: MessageEvent| {
        if let Some(text) = event.data().as_string() {
            if let Ok(msg) = serde_json::from_str::<serde_json::Value>(&text) {
                let event_name = msg
                    .get("event")
                    .and_then(|e| e.as_str())
                    .unwrap_or_default();

                if event_name == "connected" {
                    if let Some(sid) = msg
                        .get("data")
                        .and_then(|d| d.get("socket_id"))
                        .and_then(|s| s.as_str())
                    {
                        *sid_clone.borrow_mut() = Some(sid.to_string());
                    }
                    set_state(&sw_msg, ConnectionState::Connected);
                    return;
                }

                if let Some(channel_name) = msg.get("channel").and_then(|c| c.as_str()) {
                    let data = msg
                        .get("data")
                        .cloned()
                        .unwrap_or(serde_json::Value::Null);

                    let channels = ch_clone.borrow();
                    if let Some(ch) = channels.get(channel_name) {
                        for (evt, callback) in &ch.handlers {
                            if evt == event_name {
                                callback(data.clone());
                            }
                        }
                    }
                }
            }
        }
    }) as Box<dyn FnMut(MessageEvent)>);
    ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));

    // on_close — trigger reconnection
    let sw_close = state_writer.clone();
    let url_owned = url.to_string();
    let ws_rc2 = ws_rc.clone();
    let sid_close = socket_id.clone();
    let ch_close = channels.clone();
    let closures_close = closures.clone();
    let on_close = Closure::wrap(Box::new(move || {
        set_state(&sw_close, ConnectionState::Disconnected);
        start_reconnect(
            url_owned.clone(),
            ws_rc2.clone(),
            sid_close.clone(),
            ch_close.clone(),
            sw_close.clone(),
            closures_close.clone(),
        );
    }) as Box<dyn FnMut()>);
    ws.set_onclose(Some(on_close.as_ref().unchecked_ref()));

    // on_error
    let on_error = Closure::wrap(Box::new(move |_: web_sys::Event| {
        web_sys::console::error_1(&"WebSocket error".into());
    }) as Box<dyn FnMut(web_sys::Event)>);
    ws.set_onerror(Some(on_error.as_ref().unchecked_ref()));

    // Store closures to prevent GC
    let mut c = closures.borrow_mut();
    c.on_open = Some(on_open);
    c.on_message = Some(on_message);
    c.on_close = Some(on_close);
    c.on_error = Some(on_error);
}

fn resubscribe_channels(
    ws_rc: &Rc<RefCell<WebSocket>>,
    channels: &Rc<RefCell<HashMap<String, ChannelHandlers>>>,
) {
    let ws = ws_rc.borrow();
    let channels = channels.borrow();
    for channel_name in channels.keys() {
        let msg = serde_json::json!({
            "action": "subscribe",
            "channel": channel_name
        });
        let _ = ws.send_with_str(&msg.to_string());
    }
}

fn start_reconnect(
    url: String,
    ws_rc: Rc<RefCell<WebSocket>>,
    socket_id: Rc<RefCell<Option<String>>>,
    channels: Rc<RefCell<HashMap<String, ChannelHandlers>>>,
    state_writer: Rc<RefCell<Option<WriteSignal<ConnectionState>>>>,
    closures: Rc<RefCell<WsClosures>>,
) {
    wasm_bindgen_futures::spawn_local(async move {
        let mut attempt: u32 = 0;
        loop {
            attempt += 1;
            set_state(&state_writer, ConnectionState::Reconnecting(attempt));

            // Exponential backoff with jitter, capped at 30s
            let base_delay = std::cmp::min(30_000u32, 1000 * 2u32.saturating_pow(attempt - 1));
            let jitter = (js_sys::Math::random() * 1000.0) as u32;
            sleep_ms(base_delay + jitter).await;

            // Attempt new connection
            let new_ws = match WebSocket::new(&url) {
                Ok(ws) => ws,
                Err(_) => continue,
            };

            // Wait for open or error using a shared flag
            // 0 = pending, 1 = open, 2 = error
            let flag = Rc::new(Cell::new(0u8));

            let flag_open = flag.clone();
            let temp_on_open = Closure::wrap(Box::new(move || {
                flag_open.set(1);
            }) as Box<dyn FnMut()>);
            new_ws.set_onopen(Some(temp_on_open.as_ref().unchecked_ref()));

            let flag_err = flag.clone();
            let temp_on_error = Closure::wrap(Box::new(move |_: web_sys::Event| {
                flag_err.set(2);
            }) as Box<dyn FnMut(web_sys::Event)>);
            new_ws.set_onerror(Some(temp_on_error.as_ref().unchecked_ref()));

            let flag_close = flag.clone();
            let temp_on_close = Closure::wrap(Box::new(move || {
                if flag_close.get() == 0 {
                    flag_close.set(2);
                }
            }) as Box<dyn FnMut()>);
            new_ws.set_onclose(Some(temp_on_close.as_ref().unchecked_ref()));

            // Capture the "connected" message during the temp phase so
            // socket_id is set before we attach permanent handlers.
            let sid_temp = socket_id.clone();
            let temp_on_message = Closure::wrap(Box::new(move |event: MessageEvent| {
                if let Some(text) = event.data().as_string() {
                    if let Ok(msg) = serde_json::from_str::<serde_json::Value>(&text) {
                        let ev = msg.get("event").and_then(|e| e.as_str()).unwrap_or_default();
                        if ev == "connected" {
                            if let Some(sid) = msg
                                .get("data")
                                .and_then(|d| d.get("socket_id"))
                                .and_then(|s| s.as_str())
                            {
                                *sid_temp.borrow_mut() = Some(sid.to_string());
                            }
                        }
                    }
                }
            }) as Box<dyn FnMut(MessageEvent)>);
            new_ws.set_onmessage(Some(temp_on_message.as_ref().unchecked_ref()));

            // Poll for result (up to 5s)
            let mut wait = 0;
            while flag.get() == 0 && wait < 50 {
                sleep_ms(100).await;
                wait += 1;
            }

            // Keep temp closures alive until we're done checking
            drop(temp_on_open);
            drop(temp_on_error);
            drop(temp_on_close);
            drop(temp_on_message);

            if flag.get() == 1 {
                // Success — swap in the new WebSocket
                *ws_rc.borrow_mut() = new_ws;

                // Mark connected immediately (on_open already fired)
                set_state(&state_writer, ConnectionState::Connected);

                // Re-attach permanent handlers
                attach_handlers(
                    &ws_rc,
                    &socket_id,
                    &channels,
                    &state_writer,
                    &url,
                    &closures,
                );

                // Wait briefly for socket_id if not already set by temp handler
                let mut sid_wait = 0;
                while socket_id.borrow().is_none() && sid_wait < 20 {
                    sleep_ms(100).await;
                    sid_wait += 1;
                }

                // Re-subscribe to all channels
                resubscribe_channels(&ws_rc, &channels);

                break;
            } else {
                // Failed — close and retry
                let _ = new_ws.close();
            }
        }
    });
}
