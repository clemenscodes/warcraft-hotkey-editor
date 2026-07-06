mod props;
mod style;

use dioxus::prelude::*;
pub use props::HelpInlineIconProps;
use style::CLASS;
use tw_macro::assert_component;
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
