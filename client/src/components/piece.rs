use leptos::prelude::*;
use stratego::models::Piece;

use crate::app::RankStyleSignal;
use crate::config;

/// Renders a piece exactly matching the original jade/mixins/piece.jade mixin.
///
/// HTML structure:
///   <div class="piece image-{rank}-{color} [phantom]" data-rank="{rank}" data-side="{side}"></div>
///   [<div class="piece-rank image-rank-{display_rank}-{color} [phantom]" data-rank="{rank}"></div>]
///   [<div class="dead-piece-rank image-[rank-]{display_rank}-{color}" data-rank="{rank}"></div>]
#[component]
pub fn PieceView(
    piece: Piece,
    #[prop(default = false)] phantom: bool,
    #[prop(optional)] dead_piece: Option<Piece>,
) -> impl IntoView {
    let rank_style = use_context::<RankStyleSignal>()
        .map(|s| s.0.get_untracked())
        .unwrap_or(config::RankStyle::European);

    let color = match piece.side {
        0 => "red",
        1 => "blue",
        _ => "black",
    };
    let phantom_class = if phantom { " phantom" } else { "" };

    // Piece body uses internal rank (silhouette doesn't change)
    let piece_class = format!("piece image-{}-{}{}", piece.rank, color, phantom_class);

    // Rank overlay uses display rank (number changes with style)
    let display = config::display_rank(&piece.rank, rank_style);
    let show_rank = piece.rank != "B" && piece.rank != "F" && piece.rank != "U";
    let rank_class = format!(
        "piece-rank image-rank-{}-{}{}",
        display, color, phantom_class
    );

    let dead_view = dead_piece.map(|dp| {
        let dp_color = if dp.side == 0 { "red" } else { "blue" };
        let prefix = if dp.rank == "B" || dp.rank == "F" {
            ""
        } else {
            "rank-"
        };
        let dp_display = if prefix.is_empty() {
            dp.rank.as_str()
        } else {
            config::display_rank(&dp.rank, rank_style)
        };
        let dead_class = format!(
            "dead-piece-rank image-{}{}-{}",
            prefix, dp_display, dp_color
        );
        view! {
            <div class=dead_class data-rank=dp.rank.clone()></div>
        }
    });

    view! {
        <div class=piece_class
             data-rank=piece.rank.clone()
             data-side=piece.side.to_string()>
        </div>
        {show_rank.then(|| view! {
            <div class=rank_class data-rank=piece.rank.clone()></div>
        })}
        {dead_view}
    }
}
