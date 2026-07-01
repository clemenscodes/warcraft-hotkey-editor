mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

assert_component!(SystemHotkeysCategorySeparator);

/// The chevron glyph shown between two category tabs.
#[component]
pub fn SystemHotkeysCategorySeparator() -> Element {
    rsx! {
        span { class: CLASS, "\u{203A}" }
    }
}
