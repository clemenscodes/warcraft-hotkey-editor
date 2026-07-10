mod props;
mod view;

pub use view::AboveCenterTooltipView;
mod style;

use dioxus::prelude::*;
use props::AboveCenterTooltipProps;
use style::CLASS;
use tw_macro::assert_component;

/// The tooltip bubble placed above its trigger and anchored center. It owns
/// its own bubble and the positioning utilities for this placement and anchor across
/// both `@supports` bands; it shows only the message it is handed.
#[component]
pub fn AboveCenterTooltip(props: AboveCenterTooltipProps) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(AboveCenterTooltip);
