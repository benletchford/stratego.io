use leptos::prelude::*;
use stratego::models::Piece;

/// Renders a piece exactly matching the original jade/mixins/piece.jade mixin.
///
/// HTML structure:
///   <div class="piece image-{rank}-{color} [phantom]" data-rank="{rank}" data-side="{side}"></div>
///   [<div class="piece-rank image-rank-{rank}-{color} [phantom]" data-rank="{rank}"></div>]
///   [<div class="dead-piece-rank image-[rank-]{rank}-{color}" data-rank="{rank}"></div>]
#[component]
pub fn PieceView(
    piece: Piece,
    #[prop(default = false)] phantom: bool,
    #[prop(optional)] dead_piece: Option<Piece>,
) -> impl IntoView {
    let color = match piece.side {
        0 => "red",
        1 => "blue",
        _ => "black",
    };
    let rank_and_color = format!("{}-{}", piece.rank, color);
    let phantom_class = if phantom { " phantom" } else { "" };

    let piece_class = format!("piece image-{}{}", rank_and_color, phantom_class);

    let show_rank = piece.rank != "B" && piece.rank != "F" && piece.rank != "U";
    let rank_class = format!(
        "piece-rank image-rank-{}{}",
        rank_and_color, phantom_class
    );

    let dead_view = dead_piece.map(|dp| {
        let dp_color = if dp.side == 0 { "red" } else { "blue" };
        let prefix = if dp.rank == "B" || dp.rank == "F" {
            ""
        } else {
            "rank-"
        };
        let dead_class = format!(
            "dead-piece-rank image-{}{}-{}",
            prefix, dp.rank, dp_color
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
