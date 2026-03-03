use leptos::prelude::*;

/// Loading spinner with message text.
/// Matches the original loading.jade / LoadingView.coffee structure.
#[component]
pub fn Loading(
    #[prop(into)] message: Signal<String>,
) -> impl IntoView {
    view! {
        <div class="loading-view">
            <div class="loading-container">
                <div class="spinner-container">
                    <div class="spinner"></div>
                </div>
                <div class="loading-html" inner_html=move || message.get()>
                </div>
            </div>
        </div>
    }
}
