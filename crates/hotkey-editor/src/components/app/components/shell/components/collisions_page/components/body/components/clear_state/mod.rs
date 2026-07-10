mod style;

use crate::components::app::components::shell::components::shared::clear_icon::ClearIcon;
use crate::components::app::components::shell::components::shared::clear_label::ClearLabel;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ClearState);

/// The centered "all clear" state for a collision kind with no conflicts. It centers
/// its glyph and label in the available space.
#[component]
pub fn ClearState() -> Element {
    rsx! {
        section {
            class: CLASS,
            ClearIcon {}
            ClearLabel { text: "All clear." }
        }
    }
}
