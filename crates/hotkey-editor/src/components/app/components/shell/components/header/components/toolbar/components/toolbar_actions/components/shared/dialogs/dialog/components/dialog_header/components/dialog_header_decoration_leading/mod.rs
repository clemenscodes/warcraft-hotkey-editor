mod style;

use super::dialog_header_decoration::DialogHeaderDecoration;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(DialogHeaderDecorationLeading);

/// The leading side of the header: the flourish facing inward as drawn.
#[component]
pub fn DialogHeaderDecorationLeading() -> Element {
    rsx! {
        span { class: CLASS, DialogHeaderDecoration {} }
    }
}
