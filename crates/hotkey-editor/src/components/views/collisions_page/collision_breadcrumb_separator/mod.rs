mod style;
use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(CollisionBreadcrumbSeparator);

/// The "|" between two breadcrumb tabs.
#[component]
pub fn CollisionBreadcrumbSeparator() -> Element {
    rsx! { span { class: CLASS, aria_hidden: "true", "|" } }
}
