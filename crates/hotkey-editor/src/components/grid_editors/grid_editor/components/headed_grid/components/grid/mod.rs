pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use components::grid_tile::GridTile;
use style::GRID_STYLE_SHEETS;

pub use props::GridProps;

/// The command grid: a pure tile renderer. It lays out the finished tiles it is
/// handed and draws each one. It owns no behavior and no domain type. The
/// `GridEditor` builds the tiles, with their handlers and drag state, and the
/// templates preview hands it plain read-only tiles.
#[component]
pub fn Grid(props: GridProps) -> Element {
    let tiles = props.tiles;
    rsx! {
        for href in GRID_STYLE_SHEETS {
            document::Stylesheet { href }
        }
        div {
            class: "grid",
            for tile in tiles {
                GridTile { ..tile }
            }
        }
    }
}
