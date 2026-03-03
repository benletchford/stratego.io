/// Base URL for API calls. Empty string means same-origin (relative URLs).
/// Set via `API_BASE_URL` env var at compile time for cross-origin deployments.
pub fn api_base_url() -> &'static str {
    option_env!("API_BASE_URL").unwrap_or("")
}

/// WebSocket URL. Derived from `API_BASE_URL` if set, otherwise from `window.location`.
pub fn ws_url() -> String {
    let base = api_base_url();
    if base.is_empty() {
        let window = web_sys::window().unwrap();
        let location = window.location();
        let protocol = if location.protocol().unwrap() == "https:" {
            "wss:"
        } else {
            "ws:"
        };
        let host = location.host().unwrap();
        format!("{}//{}/ws", protocol, host)
    } else {
        let ws_base = base
            .replace("https://", "wss://")
            .replace("http://", "ws://");
        format!("{}/ws", ws_base)
    }
}
