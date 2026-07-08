mod style;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(BreadcrumbSeparator);
#[component]
pub fn BreadcrumbSeparator() -> Element {
    rsx! { span { class: CLASS, aria_hidden: "true", "|" } }
}
