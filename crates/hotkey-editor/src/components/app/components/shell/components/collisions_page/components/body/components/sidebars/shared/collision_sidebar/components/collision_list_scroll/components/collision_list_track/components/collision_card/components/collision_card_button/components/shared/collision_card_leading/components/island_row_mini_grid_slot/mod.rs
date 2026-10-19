mod model;
mod view;

pub use view::IslandRowMiniGridSlotView;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::mini_grid::MiniGrid;
use dioxus::prelude::*;
use model::IslandRowMiniGridSlotModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn IslandRowMiniGridSlot(props: IslandRowMiniGridSlotModel) -> Element {
    let coordinate = props.coordinate;
    rsx! {
        div {
            class: CLASS,
            MiniGrid {
                coordinate,
            }
        }
    }
}

assert_component!(IslandRowMiniGridSlot);
