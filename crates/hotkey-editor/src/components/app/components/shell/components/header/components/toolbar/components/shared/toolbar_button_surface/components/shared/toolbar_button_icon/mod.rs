mod props;
mod style;

use dioxus::prelude::*;
pub use props::ToolbarButtonIconProps;
use style::CLASS;
use tw_macro::assert_component;
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
