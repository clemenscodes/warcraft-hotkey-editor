mod data;
mod style;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PlanTitle() -> Element {
    let title = String::from(data::TITLE);
    rsx! {
        span {
            class: CLASS,
            {title}
        }
    }
}

assert_component!(PlanTitle);
