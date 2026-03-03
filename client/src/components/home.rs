use leptos::prelude::*;

use crate::app::RankStyleSignal;
use crate::config::{self, RankStyle};

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

    let rank_style = use_context::<RankStyleSignal>().unwrap().0;
    let set_rank_style = use_context::<WriteSignal<RankStyle>>().unwrap();

    let toggle_rank_style = move |_: web_sys::MouseEvent| {
        let new_style = match rank_style.get_untracked() {
            RankStyle::European => RankStyle::American,
            RankStyle::American => RankStyle::European,
        };
        set_rank_style.set(new_style);
        config::save_rank_style(new_style);
    };

    let rank_label = move || match rank_style.get() {
        RankStyle::European => "European (1 = Marshal)",
        RankStyle::American => "American (10 = Marshal)",
    };

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
            <button class="panel-button-view panel-option" on:click=toggle_rank_style>
                <div class="title">"Ranking Style"</div>
                <div class="description">{rank_label}</div>
            </button>
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
