mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::ToolbarButtonIconProps;

assert_component!(ToolbarButtonIcon);

/// The glyph inside a toolbar button.
#[component]
pub fn ToolbarButtonIcon(props: ToolbarButtonIconProps) -> Element {
    let icon = props.icon;
    rsx! {
        span {
            class: CLASS,
            aria_hidden: "true",
            dangerous_inner_html: icon,
        }
    }
}
