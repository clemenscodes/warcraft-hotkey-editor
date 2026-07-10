mod data;
mod style;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

/// The resolve plan's heading: a `span` wearing the uppercase gold heading look.
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
