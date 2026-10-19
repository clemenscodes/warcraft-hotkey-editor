mod style;

use super::shared::dialog_header_decoration::DialogHeaderDecoration;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn DialogHeaderDecorationTrailing() -> Element {
    rsx! {
        span {
            class: CLASS,
            DialogHeaderDecoration {}
        }
    }
}

assert_component!(DialogHeaderDecorationTrailing);
