mod logic;
mod props;
mod style;

use crate::assert_component;
use crate::components::grid_editors::grid_editor::components::headed_grid::components::grid::Grid;
use dioxus::prelude::*;
pub use props::MiniGridProps;
use style::CLASS;
assert_component!(MiniGrid);

/// A tiny command grid showing where a move's abilities land. It reuses the generic
/// `Grid`, shrunk by rendering it in a small query container, with each placed
/// ability drawn as a filled tile.
#[component]
pub fn MiniGrid(props: MiniGridProps) -> Element {
    let grid = logic::grid(&props);
    rsx! {
        div {
            class: CLASS,
            Grid { ..grid }
        }
    }
}
