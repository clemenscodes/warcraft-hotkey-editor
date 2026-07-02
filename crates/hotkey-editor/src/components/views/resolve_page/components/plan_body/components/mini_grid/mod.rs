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

/// A 4×3 command grid that draws each placed ability's icon into its cell, so a
/// move reads as "this ability ends up here".
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
