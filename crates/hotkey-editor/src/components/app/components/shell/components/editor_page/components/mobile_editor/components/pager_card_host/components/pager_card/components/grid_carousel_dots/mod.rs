pub mod components;
mod model;
mod view;

pub use view::GridCarouselDotsView;
mod style;

use components::carousel_dot::CarouselDot;
use dioxus::prelude::*;
use model::GridCarouselDotsModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn GridCarouselDots(props: GridCarouselDotsModel) -> Element {
    let grid_count = props.grid_count;
    let active_grid_index = props.active_grid_index;
    if grid_count <= 1 {
        return rsx! {};
    }
    rsx! {
        div {
            class: CLASS,
            for grid_index in 0..grid_count {
                CarouselDot {
                    key: "{grid_index}",
                    active: grid_index == active_grid_index,
                }
            }
        }
    }
}

assert_component!(GridCarouselDots);
