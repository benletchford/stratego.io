use leptos::prelude::*;

use crate::app::RankStyleSignal;
use crate::config::{self, RankStyle};

mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

const REPO_URL: &str = "https://github.com/benletchford/stratego.io";

#[component]
pub fn HomePage() -> impl IntoView {
    let version_label = format!("v{}", built_info::PKG_VERSION);

    let last_commit = env!("GIT_RECENT_COMMITS")
        .split("||")
        .next()
        .and_then(|line| {
            let (hash, msg) = line.split_once(' ')?;
            Some((hash.to_string(), msg.to_string()))
        });

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
                <a href=REPO_URL>"Github Repository"</a>
                <br />
                "Pieces from "
                <a href="http://vector.gissen.nl/stratego.html">"vector.gissen.nl"</a>
                <br />
                <br />
                <div class="version-info">
                    <div>{version_label}</div>
                    {last_commit.map(|(hash, msg)| {
                        let url = format!("{}/commit/{}", REPO_URL, hash);
                        let label = format!("({}) {}", hash, msg);
                        view! {
                            <div><a href={url} class="commit-link">{label}</a></div>
                        }
                    })}
                </div>
            </div>
        </div>
    }
}
