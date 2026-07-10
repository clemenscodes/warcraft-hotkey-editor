mod props;
mod style;

use crate::components::app::components::shell::components::shared::tile_face::TileFace;
use dioxus::prelude::*;
pub use props::PreviewGridProps;
use style::CLASS;
use tw_macro::assert_component;

/// The template preview's read-only command grid: the three-by-four square of
/// `TileFace` painters — the same tiles the editor draws, without any interaction.
/// A pure tile renderer that encodes the grid shape (shared with the editor and
/// mini grids via the same utility values) and draws whatever twelve tiles it is
/// handed.
#[component]
pub fn PreviewGrid(props: PreviewGridProps) -> Element {
    let tiles = props.tiles;
    rsx! {
        div { class: CLASS,
            for tile in tiles {
                TileFace { ..tile }
            }
        }
    }
}

assert_component!(PreviewGrid);
