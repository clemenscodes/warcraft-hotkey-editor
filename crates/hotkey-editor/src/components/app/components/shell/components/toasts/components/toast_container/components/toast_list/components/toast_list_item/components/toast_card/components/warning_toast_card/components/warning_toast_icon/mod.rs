mod data;
mod style;

use data::ICON;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(WarningToastIcon);

/// The circular warning triangle glyph at the leading edge of a warning toast.
#[component]
pub fn WarningToastIcon() -> Element {
    rsx! {
        span {
            class: CLASS,
            aria_hidden: "true",
            dangerous_inner_html: ICON,
        }
    }
}
