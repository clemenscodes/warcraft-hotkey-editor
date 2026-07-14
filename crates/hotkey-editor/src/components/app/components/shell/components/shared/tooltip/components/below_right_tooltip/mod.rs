mod model;
mod view;

pub use view::BelowRightTooltipView;
mod style;

use dioxus::prelude::*;
use model::BelowRightTooltipModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn BelowRightTooltip(props: BelowRightTooltipModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(BelowRightTooltip);
