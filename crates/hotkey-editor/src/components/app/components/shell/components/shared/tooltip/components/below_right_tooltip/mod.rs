mod props;
mod style;

use dioxus::prelude::*;
pub use props::BelowRightTooltipProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(BelowRightTooltip);

/// The tooltip bubble placed below its trigger and anchored right. It owns
/// its own bubble and the positioning utilities for this placement and anchor across
/// both `@supports` bands; it shows only the message it is handed.
#[component]
pub fn BelowRightTooltip(props: BelowRightTooltipProps) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}
