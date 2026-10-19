mod model;
mod view;

pub use view::BelowCenterTooltipView;
mod style;

use dioxus::prelude::*;
use model::BelowCenterTooltipModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn BelowCenterTooltip(props: BelowCenterTooltipModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(BelowCenterTooltip);
