mod props;
mod view;

pub use view::AboveLeftTooltipView;
mod style;

use dioxus::prelude::*;
use props::AboveLeftTooltipProps;
use style::CLASS;
use tw_macro::assert_component;

/// The tooltip bubble placed above its trigger and anchored left. It owns
/// its own bubble and the positioning utilities for this placement and anchor across
/// both `@supports` bands; it shows only the message it is handed.
#[component]
pub fn AboveLeftTooltip(props: AboveLeftTooltipProps) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(AboveLeftTooltip);
