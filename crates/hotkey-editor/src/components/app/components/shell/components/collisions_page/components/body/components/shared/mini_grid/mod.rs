mod logic;
mod props;
mod view;

pub use view::MiniGridView;
mod style;

use crate::components::app::components::shell::components::shared::mini_grid_frame::MiniGridFrame;
use dioxus::prelude::*;
use props::MiniGridProps;
use style::CLASS;
use tw_macro::assert_component;

/// A tiny command grid marking one coordinate. A thin wrapper that owns the
/// collisions page's fixed-width, hairline-radius outer box and highlights the
/// given coordinate's tile before handing the built grid to the shared `MiniGrid`
/// frame. Shared by the island sidebar cards and the detail headers.
#[component]
pub fn MiniGrid(props: MiniGridProps) -> Element {
    let tiles = logic::grid(&props);
    rsx! {
        div {
            class: CLASS,
            MiniGridFrame { tiles }
        }
    }
}

assert_component!(MiniGrid);
