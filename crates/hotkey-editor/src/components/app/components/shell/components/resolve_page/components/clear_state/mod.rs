mod style;
use crate::components::app::components::shell::components::shared::clear_icon::ClearIcon;
use crate::components::app::components::shell::components::shared::clear_label::ClearLabel;
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
            ClearLabel { text: "Nothing to resolve." }
        }
    }
}
