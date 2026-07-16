mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn InactiveCarouselDot() -> Element {
    rsx! {
        span {
            class: CLASS,
        }
    }
}

assert_component!(InactiveCarouselDot);
