pub mod components;
mod logic;
mod props;
mod style;

use crate::assert_component;
use components::mini_cell::MiniCell;
use dioxus::prelude::*;
use logic::cells;
pub use props::MiniGridProps;
use style::CLASS;
assert_component!(MiniGrid);

/// A tiny empty 4×3 command grid with only the island's conflicting cell
/// highlighted. Shared by the sidebar island cards and the detail header.
#[component]
pub fn MiniGrid(props: MiniGridProps) -> Element {
    let cells = cells(&props);
    rsx! {
        div {
            class: CLASS,
            for cell in cells {
                MiniCell { ..cell }
            }
        }
    }
}
