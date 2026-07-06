mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(SystemHotkeysCategorySeparator);

/// The chevron glyph shown between two category tabs.
#[component]
pub fn SystemHotkeysCategorySeparator() -> Element {
    rsx! {
        span { class: CLASS, "\u{203A}" }
    }
}
