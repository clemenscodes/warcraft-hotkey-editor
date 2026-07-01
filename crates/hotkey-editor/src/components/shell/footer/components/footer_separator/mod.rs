mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

assert_component!(FooterSeparator);

#[component]
pub fn FooterSeparator() -> Element {
    rsx! {
        span {
            class: CLASS,
            aria_hidden: "true",
            "\u{00b7}"
        }
    }
}
