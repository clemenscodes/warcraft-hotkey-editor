mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(FooterSeparator);

#[component]
pub fn FooterSeparator() -> Element {
    rsx! {
        span { class: CLASS, aria_hidden: "true", "\u{00b7}" }
    }
}
