mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::ToolbarButtonIconProps;
use style::CLASS;
assert_component!(ToolbarButtonIcon);

/// The glyph inside a toolbar button.
#[component]
pub fn ToolbarButtonIcon(props: ToolbarButtonIconProps) -> Element {
    let icon = props.icon;
    rsx! {
        span { class: CLASS, aria_hidden: "true", dangerous_inner_html: icon }
    }
}
