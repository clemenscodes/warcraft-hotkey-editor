mod model;
mod view;

pub use view::BelowLeftTooltipView;
mod style;

use dioxus::prelude::*;
use model::BelowLeftTooltipModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn BelowLeftTooltip(props: BelowLeftTooltipModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(BelowLeftTooltip);
