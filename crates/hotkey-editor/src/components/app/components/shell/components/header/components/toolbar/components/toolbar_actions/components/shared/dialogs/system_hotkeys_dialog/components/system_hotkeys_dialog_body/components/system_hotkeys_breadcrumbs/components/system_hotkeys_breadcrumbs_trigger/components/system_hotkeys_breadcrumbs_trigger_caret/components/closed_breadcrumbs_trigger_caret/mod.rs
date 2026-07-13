mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

/// The trigger caret in its resting, upright look. Rendered while the dropdown is
/// closed.
#[component]
pub fn ClosedBreadcrumbsTriggerCaret() -> Element {
    rsx! {
        span { class: CLASS, aria_hidden: "true", "\u{25BE}" }
    }
}

assert_component!(ClosedBreadcrumbsTriggerCaret);
