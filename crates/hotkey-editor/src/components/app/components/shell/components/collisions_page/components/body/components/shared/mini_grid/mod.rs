mod logic;
mod props;
mod style;

use crate::components::app::components::shell::components::shared::mini_grid_frame::MiniGridFrame;
use dioxus::prelude::*;
pub use props::MiniGridProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(MiniGrid);

/// A tiny command grid marking one coordinate. A thin wrapper that owns the
/// collisions page's fixed-width, hairline-radius outer box and highlights the
/// given coordinate's tile before handing the built grid to the shared `MiniGrid`
/// frame. Shared by the island sidebar cards and the detail headers.
#[component]
pub fn MiniGrid(props: MiniGridProps) -> Element {
    let grid = logic::grid(&props);
    rsx! {
        div {
            class: CLASS,
            MiniGridFrame { grid }
        }
    }
}
