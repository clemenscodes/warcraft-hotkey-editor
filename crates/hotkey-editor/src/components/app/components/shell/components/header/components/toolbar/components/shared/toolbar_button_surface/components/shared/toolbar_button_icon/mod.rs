mod props;
mod style;

use dioxus::prelude::*;
use props::ToolbarButtonIconProps;
use style::CLASS;
use tw_macro::assert_component;

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

assert_component!(ToolbarButtonIcon);
