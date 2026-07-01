pub mod components;
mod logic;
mod props;
mod style;

use crate::assert_component;
use components::island_mini_cell::IslandMiniCell;
use dioxus::prelude::*;
use logic::cells;
pub use props::IslandMiniGridProps;
use style::CLASS;
assert_component!(IslandMiniGrid);

/// A tiny empty 4×3 command grid with only the island's conflicting cell
/// highlighted. Shared by the sidebar island cards and the detail header.
#[component]
pub fn IslandMiniGrid(props: IslandMiniGridProps) -> Element {
    let cells = cells(&props);
    rsx! {
        div {
            class: CLASS,
            for cell in cells {
                IslandMiniCell { ..cell }
            }
        }
    }
}
