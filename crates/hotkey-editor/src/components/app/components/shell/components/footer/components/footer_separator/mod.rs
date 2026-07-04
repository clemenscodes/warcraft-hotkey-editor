mod style;

use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(FooterSeparator);

#[component]
pub fn FooterSeparator() -> Element {
    rsx! {
        span { class: CLASS, aria_hidden: "true", "\u{00b7}" }
    }
}
