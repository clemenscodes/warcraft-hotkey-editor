mod model;
mod view;

mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::shared::mini_grid::MiniGrid;
use dioxus::prelude::*;
use model::IslandDetailMiniGridSlotModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn IslandDetailMiniGridSlot(props: IslandDetailMiniGridSlotModel) -> Element {
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

assert_component!(IslandDetailMiniGridSlot);
