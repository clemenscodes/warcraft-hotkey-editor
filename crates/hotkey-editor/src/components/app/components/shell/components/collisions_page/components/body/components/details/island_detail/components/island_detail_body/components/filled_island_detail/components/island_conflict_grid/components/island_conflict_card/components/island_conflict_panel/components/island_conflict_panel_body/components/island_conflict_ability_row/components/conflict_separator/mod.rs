mod data;
mod style;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ConflictSeparator() -> Element {
    rsx! {
        span {
            class: CLASS,
            aria_hidden: "true",
            {data::SEPARATOR}
        }
    }
}

assert_component!(ConflictSeparator);
