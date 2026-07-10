mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn FooterSeparator() -> Element {
    rsx! {
        span { class: CLASS, aria_hidden: "true", "\u{00b7}" }
    }
}

assert_component!(FooterSeparator);
