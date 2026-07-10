mod props;
mod style;

use dioxus::prelude::*;
use props::BelowLeftTooltipProps;
use style::CLASS;
use tw_macro::assert_component;

/// The tooltip bubble placed below its trigger and anchored left. It owns
/// its own bubble and the positioning utilities for this placement and anchor across
/// both `@supports` bands; it shows only the message it is handed.
#[component]
pub fn BelowLeftTooltip(props: BelowLeftTooltipProps) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(BelowLeftTooltip);
