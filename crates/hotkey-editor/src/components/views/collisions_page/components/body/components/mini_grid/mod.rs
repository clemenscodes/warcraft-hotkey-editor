pub mod components;
mod logic;
mod props;
mod style;

use crate::assert_component;
use components::mini_tile::MiniTile;
use dioxus::prelude::*;
use logic::tiles;
pub use props::MiniGridProps;
use style::CLASS;
assert_component!(MiniGrid);

/// A tiny 4×3 command grid with only the given coordinate's tile highlighted.
/// Shared by the sidebar island cards and the detail header.
#[component]
pub fn MiniGrid(props: MiniGridProps) -> Element {
    let tiles = tiles(&props);
    rsx! {
        div {
            class: CLASS,
            for tile in tiles {
                MiniTile { ..tile }
            }
        }
    }
}
