mod props;
mod style;

use crate::components::app::components::shell::components::shared::grid_tile::GridTile;
use dioxus::prelude::*;
pub use props::MiniGridProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(MiniGrid);

/// The shrunk read-only command grid: the three-by-four square of inert base
/// `GridTile`s. A pure tile renderer that encodes the grid shape (shared with the
/// editor and preview grids via the same utility values) and draws whatever twelve
/// tiles its frame hands it.
#[component]
pub fn MiniGrid(props: MiniGridProps) -> Element {
    let tiles = props.tiles;
    rsx! {
        div { class: CLASS,
            for tile in tiles {
                GridTile { ..tile }
            }
        }
    }
}
