pub mod components;
mod logic;
mod props;
mod style;
use components::resolve_mini_cell::ResolveMiniCell;
use crate::assert_component;
use dioxus::prelude::*;
use logic::cells;
pub use props::ResolveMiniGridProps;
use style::CLASS;
assert_component!(ResolveMiniGrid);

/// A 4×3 command grid that draws each placed ability's icon into its cell, so a
/// move reads as "this ability ends up here".
#[component]
pub fn ResolveMiniGrid(props: ResolveMiniGridProps) -> Element {
    let cells = cells(&props);
    rsx! {
        div {
            class: CLASS,
            for cell in cells {
                ResolveMiniCell { ..cell }
            }
        }
    }
}
