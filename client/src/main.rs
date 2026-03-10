mod api;
mod app;
mod components;
mod config;
mod poll;

fn main() {
    leptos::mount::mount_to_body(app::App);
}
