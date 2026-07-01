mod style;
use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(ResolveBreadcrumbSeparator);
#[component]
pub fn ResolveBreadcrumbSeparator() -> Element {
    rsx! { span { class: CLASS, aria_hidden: "true", "|" } }
}
