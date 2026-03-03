use leptos::prelude::*;

mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[component]
pub fn HomePage() -> impl IntoView {
    let version = format!(
        "v{}{}",
        built_info::PKG_VERSION,
        built_info::GIT_COMMIT_HASH_SHORT
            .map(|h| format!(" ({})", h))
            .unwrap_or_default()
    );

    view! {
        <div class="home-view panel">
            <a class="panel-link-view panel-option" href="/setup/pool">
                <div class="title">"Online Stratego"</div>
                <div class="description">"Get matched with someone online."</div>
            </a>
            <a class="panel-link-view panel-option" href="/setup/create">
                <div class="title">"Play with a friend"</div>
                <div class="description">"Start a private game with a friend."</div>
            </a>
            <div class="panel-textbox-view panel-textbox">
                "App by "
                <a href="https://benletchford.com">"Ben Letchford"</a>
                <br />
                <br />
                <a href="https://github.com/benletchford/stratego.io">"Github Repository"</a>
                <br />
                "Pieces from "
                <a href="http://vector.gissen.nl/stratego.html">"vector.gissen.nl"</a>
                <br />
                <br />
                <span class="version-info">
                    {version}
                </span>
            </div>
        </div>
    }
}
