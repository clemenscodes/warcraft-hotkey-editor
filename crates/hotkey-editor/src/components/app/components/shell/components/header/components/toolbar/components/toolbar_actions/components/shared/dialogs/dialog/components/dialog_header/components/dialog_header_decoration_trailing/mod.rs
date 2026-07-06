mod style;

use super::dialog_header_decoration::DialogHeaderDecoration;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(DialogHeaderDecorationTrailing);

/// The trailing side of the header: the same flourish mirrored to face inward.
#[component]
pub fn DialogHeaderDecorationTrailing() -> Element {
    rsx! {
        span { class: CLASS, DialogHeaderDecoration {} }
    }
}
