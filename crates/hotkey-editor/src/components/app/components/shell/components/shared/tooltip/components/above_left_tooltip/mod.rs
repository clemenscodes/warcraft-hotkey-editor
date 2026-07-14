mod model;
mod view;

pub use view::AboveLeftTooltipView;
mod style;

use dioxus::prelude::*;
use model::AboveLeftTooltipModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn AboveLeftTooltip(props: AboveLeftTooltipModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(AboveLeftTooltip);
