pub mod components;
mod model;
mod view;

pub use view::MiniGridFrameView;
mod style;

use components::mini_grid::MiniGrid;
use dioxus::prelude::*;
use model::MiniGridFrameModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn MiniGridFrame(props: MiniGridFrameModel) -> Element {
    let tiles = props.tiles;
    rsx! {
        div {
            class: CLASS,
            MiniGrid {
                tiles,
            }
        }
    }
}

assert_component!(MiniGridFrame);
