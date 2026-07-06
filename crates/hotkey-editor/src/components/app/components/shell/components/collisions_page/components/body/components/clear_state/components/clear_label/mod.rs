mod style;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ClearLabel);
#[component]
pub fn ClearLabel() -> Element {
    rsx! { p { class: CLASS, "All clear." } }
}
