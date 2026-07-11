mod model;
mod presentation;
mod view;

pub use view::MiniGridView;
mod style;

use crate::components::app::components::shell::components::shared::mini_grid_frame::MiniGridFrame;
use dioxus::prelude::*;
use model::MiniGridModel;
use style::CLASS;
use tw_macro::assert_component;

/// A tiny command grid showing where a move's abilities land. A thin wrapper that
/// owns the resolve page's outer box — full-width and control-radius — and shapes
/// each placed ability into a filled tile before handing the built grid to the
/// shared `MiniGrid` frame.
#[component]
pub fn MiniGrid(props: MiniGridModel) -> Element {
    let tiles = presentation::grid(&props);
    rsx! {
        div {
            class: CLASS,
            MiniGridFrame { tiles }
        }
    }
}

assert_component!(MiniGrid);
