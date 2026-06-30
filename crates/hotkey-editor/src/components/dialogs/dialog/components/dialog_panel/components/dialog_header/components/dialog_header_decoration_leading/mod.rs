mod style;

use dioxus::prelude::*;

use super::dialog_header_decoration::DialogHeaderDecoration;
use crate::assert_component;
use style::CLASS;

assert_component!(DialogHeaderDecorationLeading);

/// The leading side of the header: the flourish facing inward as drawn.
#[component]
pub fn DialogHeaderDecorationLeading() -> Element {
    rsx! {
        span {
            class: CLASS,
            DialogHeaderDecoration {}
        }
    }
}
