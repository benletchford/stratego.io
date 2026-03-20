mod api;
mod app;
mod components;
mod config;
pub(crate) mod notify;
mod poll;

fn main() {
    leptos::mount::mount_to_body(app::App);
}
