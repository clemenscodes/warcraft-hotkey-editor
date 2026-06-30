mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::HelpLegendIconProps;

assert_component!(HelpLegendIcon);

/// The framed toolbar glyph in a legend row. A leaf: the row passes which icon
/// to draw.
#[component]
pub fn HelpLegendIcon(props: HelpLegendIconProps) -> Element {
    let icon = props.icon;
    rsx! {
        span {
            class: CLASS,
            aria_hidden: "true",
            dangerous_inner_html: icon,
        }
    }
}
