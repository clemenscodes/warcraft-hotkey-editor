mod style;
use crate::assert_component;
use dioxus::prelude::*;
use style::CLASS;
assert_component!(PlanTitle);
#[component]
pub fn PlanTitle() -> Element {
    rsx! { span { class: CLASS, "Cascade Plan" } }
}
