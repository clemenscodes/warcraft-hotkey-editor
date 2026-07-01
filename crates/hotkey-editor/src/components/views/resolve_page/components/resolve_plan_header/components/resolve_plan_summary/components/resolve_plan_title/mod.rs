mod style;
use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(ResolvePlanTitle);
#[component]
pub fn ResolvePlanTitle() -> Element {
    rsx! { span { class: CLASS, "Cascade Plan" } }
}
