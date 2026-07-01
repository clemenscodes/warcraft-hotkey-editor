pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::grid_tile::GridTile;
use dioxus::prelude::*;
pub use props::GridProps;
use style::CLASS;
assert_component!(Grid);

/// The command grid: a pure tile renderer. It lays out the finished tiles it is
/// handed and draws each one. It owns no behavior and no domain type. The
/// `GridEditor` builds the tiles, with their handlers and drag state, and the
/// templates preview hands it plain read-only tiles.
#[component]
pub fn Grid(props: GridProps) -> Element {
    let tiles = props.tiles;
    rsx! {
        div { class: CLASS,
            for tile in tiles {
                GridTile { ..tile }
            }
        }
    }
}
