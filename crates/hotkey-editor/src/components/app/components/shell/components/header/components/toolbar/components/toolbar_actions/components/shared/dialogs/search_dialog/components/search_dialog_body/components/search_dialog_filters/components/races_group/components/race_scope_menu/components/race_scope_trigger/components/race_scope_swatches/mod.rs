mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn RaceScopeSwatches() -> Element {
    rsx! {
        span {
            class: CLASS,
            span {}
            span {}
            span {}
            span {}
            span {}
        }
    }
}

assert_component!(RaceScopeSwatches);
