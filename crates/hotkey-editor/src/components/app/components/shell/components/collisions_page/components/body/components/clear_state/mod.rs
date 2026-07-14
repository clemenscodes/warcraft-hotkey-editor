mod style;

use crate::components::app::components::shell::components::shared::clear_icon::ClearIcon;
use crate::components::app::components::shell::components::shared::clear_label::ClearLabel;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ClearState() -> Element {
    rsx! {
        section {
            class: CLASS,
            ClearIcon {}
            ClearLabel {
                text: "All clear.",
            }
        }
    }
}

assert_component!(ClearState);
