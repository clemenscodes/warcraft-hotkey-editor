pub mod components;
mod style;
use components::clear_icon::ClearIcon;
use components::clear_label::ClearLabel;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ClearState);

/// The "nothing to resolve" state shown when a file is loaded but conflict-free.
#[component]
pub fn ClearState() -> Element {
    rsx! {
        section {
            class: CLASS,
            "data-resolve-state": "clear",
            ClearIcon {}
            ClearLabel {}
        }
    }
}
