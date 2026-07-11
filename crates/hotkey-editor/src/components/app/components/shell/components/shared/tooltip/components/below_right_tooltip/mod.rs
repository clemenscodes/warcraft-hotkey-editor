mod model;
mod view;

pub use view::BelowRightTooltipView;
mod style;

use dioxus::prelude::*;
use model::BelowRightTooltipModel;
use style::CLASS;
use tw_macro::assert_component;

/// The tooltip bubble placed below its trigger and anchored right. It owns
/// its own bubble and the positioning utilities for this placement and anchor across
/// both `@supports` bands; it shows only the message it is handed.
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
