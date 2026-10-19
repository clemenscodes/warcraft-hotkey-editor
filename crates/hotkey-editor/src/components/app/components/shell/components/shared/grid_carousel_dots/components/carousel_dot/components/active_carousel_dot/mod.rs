mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ActiveCarouselDot() -> Element {
    rsx! {
        span {
            class: CLASS,
        }
    }
}

assert_component!(ActiveCarouselDot);
