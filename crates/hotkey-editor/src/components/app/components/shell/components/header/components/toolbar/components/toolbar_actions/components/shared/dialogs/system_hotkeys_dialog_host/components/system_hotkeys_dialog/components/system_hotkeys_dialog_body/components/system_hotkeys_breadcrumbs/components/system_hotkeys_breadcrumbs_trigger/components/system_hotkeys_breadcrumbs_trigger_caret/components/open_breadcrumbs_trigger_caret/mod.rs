mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

/// The trigger caret in its open, flipped look. Rendered while the dropdown is open.
#[component]
pub fn OpenBreadcrumbsTriggerCaret() -> Element {
    rsx! {
        span { class: CLASS, aria_hidden: "true", "\u{25BE}" }
    }
}

assert_component!(OpenBreadcrumbsTriggerCaret);
