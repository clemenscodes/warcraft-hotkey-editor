mod logic;
mod props;
mod style;

use crate::components::app::components::shell::components::shared::mini_grid_frame::MiniGridFrame;
use dioxus::prelude::*;
pub use props::MiniGridProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(MiniGrid);

/// A tiny command grid showing where a move's abilities land. A thin wrapper that
/// owns the resolve page's outer box — full-width and control-radius — and shapes
/// each placed ability into a filled tile before handing the built grid to the
/// shared `MiniGrid` frame.
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
