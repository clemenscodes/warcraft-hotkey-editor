mod style;
use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(ClearLabel);
#[component]
pub fn ClearLabel() -> Element {
    rsx! { p { class: CLASS, "All clear." } }
}
