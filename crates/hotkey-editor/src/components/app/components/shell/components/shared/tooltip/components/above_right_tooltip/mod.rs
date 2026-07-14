mod model;
mod view;

pub use view::AboveRightTooltipView;
mod style;

use dioxus::prelude::*;
use model::AboveRightTooltipModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn AboveRightTooltip(props: AboveRightTooltipModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(AboveRightTooltip);
