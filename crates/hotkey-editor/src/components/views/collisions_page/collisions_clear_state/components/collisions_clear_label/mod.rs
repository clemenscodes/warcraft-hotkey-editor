mod style;
use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(CollisionsClearLabel);
#[component]
pub fn CollisionsClearLabel() -> Element {
    rsx! { p { class: CLASS, "All clear." } }
}
