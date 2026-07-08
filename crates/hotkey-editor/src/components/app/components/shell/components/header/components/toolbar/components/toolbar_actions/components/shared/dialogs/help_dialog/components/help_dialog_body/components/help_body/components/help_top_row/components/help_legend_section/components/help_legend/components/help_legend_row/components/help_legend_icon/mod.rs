mod props;
mod style;

use dioxus::prelude::*;
pub use props::HelpLegendIconProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(HelpLegendIcon);

/// The framed toolbar glyph in a legend row. A leaf: the row passes which icon
/// to draw.
#[component]
pub fn HelpLegendIcon(props: HelpLegendIconProps) -> Element {
    let icon = props.icon;
    rsx! {
        span { class: CLASS, aria_hidden: "true", dangerous_inner_html: icon }
    }
}
