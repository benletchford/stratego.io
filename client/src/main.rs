mod api;
mod app;
mod components;
mod config;
mod ws;

fn main() {
    leptos::mount::mount_to_body(app::App);
}
