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

#[component]
pub fn MiniGrid(props: MiniGridModel) -> Element {
    let tiles = presentation::grid(&props);
    rsx! {
        div {
            class: CLASS,
            MiniGridFrame {
                tiles,
            }
        }
    }
}

assert_component!(MiniGrid);
