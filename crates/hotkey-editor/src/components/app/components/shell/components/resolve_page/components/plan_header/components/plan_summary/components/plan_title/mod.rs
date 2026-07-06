mod style;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(PlanTitle);
#[component]
pub fn PlanTitle() -> Element {
    rsx! { span { class: CLASS, "Cascade Plan" } }
}
