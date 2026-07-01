mod style;
use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(ResolveUnresolvedTitle);
#[component]
pub fn ResolveUnresolvedTitle() -> Element {
    rsx! { span { class: CLASS, "Unresolved" } }
}
