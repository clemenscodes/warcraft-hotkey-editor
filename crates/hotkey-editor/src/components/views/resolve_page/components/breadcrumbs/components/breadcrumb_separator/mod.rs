mod style;
use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(BreadcrumbSeparator);
#[component]
pub fn BreadcrumbSeparator() -> Element {
    rsx! { span { class: CLASS, aria_hidden: "true", "|" } }
}
