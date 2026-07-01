mod style;

use super::dialog_header_decoration::DialogHeaderDecoration;
use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(DialogHeaderDecorationTrailing);

/// The trailing side of the header: the same flourish mirrored to face inward.
#[component]
pub fn DialogHeaderDecorationTrailing() -> Element {
    rsx! {
        span { class: CLASS, DialogHeaderDecoration {} }
    }
}
