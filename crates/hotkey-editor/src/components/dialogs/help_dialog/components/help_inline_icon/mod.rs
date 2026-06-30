mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::HelpInlineIconProps;

assert_component!(HelpInlineIcon);

/// A toolbar glyph dropped inline into a workflow sentence. A leaf: the step
/// passes which icon to draw.
#[component]
pub fn HelpInlineIcon(props: HelpInlineIconProps) -> Element {
    rsx! {
        span {
            class: CLASS,
            aria_hidden: "true",
            dangerous_inner_html: props.icon,
        }
    }
}
