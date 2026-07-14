mod style;

use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ClosedBreadcrumbsTriggerCaret() -> Element {
    rsx! {
        span {
            class: CLASS,
            aria_hidden: "true",
            "\u{25BE}"
        }
    }
}

assert_component!(ClosedBreadcrumbsTriggerCaret);
