mod data;
mod style;
use crate::components::app::components::shell::components::shared::gold_heading::{
    GoldHeading, GoldHeadingProps, GoldHeadingVariant,
};
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(PlanTitle);
#[component]
pub fn PlanTitle() -> Element {
    let title = String::from(data::TITLE);
    let heading = GoldHeadingProps {
        title,
        variant: GoldHeadingVariant::Section,
    };
    rsx! {
        span {
            class: CLASS,
            GoldHeading { ..heading }
        }
    }
}
