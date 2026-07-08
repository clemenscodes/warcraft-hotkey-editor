mod data;
mod style;

use data::ICON;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(SuccessToastIcon);

/// The circular checkmark glyph at the leading edge of a success toast.
#[component]
pub fn SuccessToastIcon() -> Element {
    rsx! {
        span {
            class: CLASS,
            aria_hidden: "true",
            dangerous_inner_html: ICON,
        }
    }
}
