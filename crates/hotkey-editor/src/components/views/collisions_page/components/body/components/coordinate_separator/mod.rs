mod style;
use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(CoordinateSeparator);
/// The "·" separator between the column and row coordinates (shown only when the
/// coords sit on one line).
#[component]
pub fn CoordinateSeparator() -> Element {
    rsx! { span { class: CLASS, "\u{00b7}" } }
}
