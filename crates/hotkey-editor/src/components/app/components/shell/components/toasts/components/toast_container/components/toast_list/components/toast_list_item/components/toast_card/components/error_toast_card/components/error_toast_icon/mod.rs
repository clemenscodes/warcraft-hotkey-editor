mod data;
mod style;

use data::ICON;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ErrorToastIcon);

/// The circular cross glyph at the leading edge of a error toast.
#[component]
pub fn ErrorToastIcon() -> Element {
    rsx! {
        span {
            class: CLASS,
            aria_hidden: "true",
            dangerous_inner_html: ICON,
        }
    }
}
