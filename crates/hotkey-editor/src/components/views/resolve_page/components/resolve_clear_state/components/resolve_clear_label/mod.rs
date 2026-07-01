mod style;
use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(ResolveClearLabel);
#[component]
pub fn ResolveClearLabel() -> Element {
    rsx! { p { class: CLASS, "Nothing to resolve." } }
}
