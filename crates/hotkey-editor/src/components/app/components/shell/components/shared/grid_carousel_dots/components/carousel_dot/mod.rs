pub mod components;
mod model;
mod view;

pub use view::CarouselDotView;

use components::active_carousel_dot::ActiveCarouselDot;
use components::inactive_carousel_dot::InactiveCarouselDot;
use dioxus::prelude::*;
use model::CarouselDotModel;
use tw_macro::assert_component;

#[component]
pub fn CarouselDot(props: CarouselDotModel) -> Element {
    match props.active {
        true => rsx! {
            ActiveCarouselDot {}
        },
        false => rsx! {
            InactiveCarouselDot {}
        },
    }
}

assert_component!(CarouselDot);
