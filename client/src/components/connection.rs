use leptos::prelude::*;

/// Connection indicator — no longer needed with polling (no persistent connection).
/// Kept as an empty component so existing references don't break.
#[component]
pub fn ConnectionIndicator() -> impl IntoView {
    view! { <div></div> }
}
