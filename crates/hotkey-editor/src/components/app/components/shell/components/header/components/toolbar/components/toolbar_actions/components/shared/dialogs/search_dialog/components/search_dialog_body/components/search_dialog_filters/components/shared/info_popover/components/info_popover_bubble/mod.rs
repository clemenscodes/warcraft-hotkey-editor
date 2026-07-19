mod model;
mod view;

pub use view::InfoPopoverBubbleView;
mod style;

use dioxus::prelude::*;
use model::InfoPopoverBubbleModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn InfoPopoverBubble(props: InfoPopoverBubbleModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            role: "tooltip",
            {text}
        }
    }
}

assert_component!(InfoPopoverBubble);
