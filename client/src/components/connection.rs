use leptos::prelude::*;

use crate::ws::{ConnectionState, ConnectionStateSignal};

#[component]
pub fn ConnectionIndicator() -> impl IntoView {
    let conn_signal = use_context::<ConnectionStateSignal>();

    move || {
        let state = conn_signal
            .as_ref()
            .map(|s| s.0.get())
            .unwrap_or(ConnectionState::Idle);

        match state {
            ConnectionState::Idle => view! {
                <div class="ws-indicator ws-idle"></div>
            }
            .into_any(),
            ConnectionState::Connected => view! {
                <div class="ws-indicator ws-connected">
                    <div class="ws-dot"></div>
                </div>
            }
            .into_any(),
            ConnectionState::Disconnected => view! {
                <div class="ws-indicator ws-disconnected">
                    <div class="ws-dot"></div>
                    <span class="ws-label">"Connection lost"</span>
                </div>
            }
            .into_any(),
            ConnectionState::Reconnecting(attempt) => view! {
                <div class="ws-indicator ws-reconnecting">
                    <div class="ws-dot"></div>
                    <span class="ws-label">
                        {format!("Reconnecting... (attempt {})", attempt)}
                    </span>
                </div>
            }
            .into_any(),
        }
    }
}
