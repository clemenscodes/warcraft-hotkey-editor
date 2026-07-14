mod style;
use crate::components::app::components::shell::components::shared::clear_icon::ClearIcon;
use crate::components::app::components::shell::components::shared::clear_label::ClearLabel;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

/// The "nothing to resolve" state shown when a file is loaded but conflict-free. It
/// centers its glyph and label in the available space.
#[component]
pub fn ClearState() -> Element {
    rsx! {
        section {
            class: CLASS,
            ClearIcon {
            


            }
            ClearLabel {
                text: "Nothing to resolve.",
            }
        }
    }
}

assert_component!(ClearState);
