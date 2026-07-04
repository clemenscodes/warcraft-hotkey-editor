pub mod components;
mod grid_tile_kind;
mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use grid_tile_kind::GridTileKind;
pub use props::GridProps;
use style::CLASS;
assert_component!(Grid);

/// The command grid: a pure tile renderer that encodes the three-by-four
/// tile-square shape once and nothing else. It owns no behavior and no domain
/// type. Generic over the [`GridTileKind`] that renders each cell — the editor
/// binds an interactive tile, the mini grid a highlighted read-only tile — so the
/// shape is written once and every extension inherits it.
#[component]
pub fn Grid<B: GridTileKind>(props: GridProps<B>) -> Element {
    let tiles = props.tiles;
    rsx! {
        div { class: CLASS,
            for tile in tiles {
                {B::tile(tile)}
            }
        }
    }
}
