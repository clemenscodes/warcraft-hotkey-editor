mod model;
mod view;

pub use view::AboveCenterTooltipView;
mod style;

use dioxus::prelude::*;
use model::AboveCenterTooltipModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn AboveCenterTooltip(props: AboveCenterTooltipModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(AboveCenterTooltip);
