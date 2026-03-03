use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::components::board::Board;
use crate::components::connection::ConnectionIndicator;
use crate::components::home::HomePage;
use crate::components::overlay::OverlayGraphics;
use crate::components::setup::SetupPage;
use crate::components::game::PlayPage;
use crate::ws::{ConnectionState, ConnectionStateSignal};

#[component]
pub fn App() -> impl IntoView {
    let (conn_state, set_conn_state) = signal(ConnectionState::Idle);
    provide_context(ConnectionStateSignal(conn_state));
    provide_context(set_conn_state);

    view! {
        <Board>
            <Router>
                <Routes fallback=|| view! { <p>"Not found"</p> }>
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/setup/create") view=move || view! { <SetupPage mode="create" hash="" /> } />
                    <Route path=path!("/setup/join/:hash") view=SetupJoinWrapper />
                    <Route path=path!("/setup/pool") view=move || view! { <SetupPage mode="pool" hash="" /> } />
                    <Route path=path!("/play/:hash") view=PlayWrapper />
                </Routes>
            </Router>
        </Board>
        <OverlayGraphics />
        <ConnectionIndicator />
    }
}

#[component]
fn SetupJoinWrapper() -> impl IntoView {
    let params = leptos_router::hooks::use_params_map();
    let hash = move || {
        params.with(|p| p.get("hash").unwrap_or_default().to_string())
    };
    view! {
        <SetupPage mode="join" hash=hash() />
    }
}

#[component]
fn PlayWrapper() -> impl IntoView {
    let params = leptos_router::hooks::use_params_map();
    // Return PlayPage from a reactive closure so it re-mounts when the
    // route parameter changes (e.g. /play/pool → /play/{player_hash}).
    move || {
        let hash = params.with(|p| p.get("hash").unwrap_or_default().to_string());
        view! { <PlayPage hash=hash /> }
    }
}
