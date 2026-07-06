mod style;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(BreadcrumbSeparator);

/// The "|" between two breadcrumb tabs.
#[component]
pub fn BreadcrumbSeparator() -> Element {
    rsx! { span { class: CLASS, aria_hidden: "true", "|" } }
}
