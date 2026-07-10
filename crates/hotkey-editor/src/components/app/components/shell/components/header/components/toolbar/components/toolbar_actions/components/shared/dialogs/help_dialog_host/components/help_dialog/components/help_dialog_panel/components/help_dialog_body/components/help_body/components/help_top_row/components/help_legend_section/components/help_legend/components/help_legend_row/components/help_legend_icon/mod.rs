mod props;
mod view;

pub use view::HelpLegendIconView;
mod style;

use dioxus::prelude::*;
use props::HelpLegendIconProps;
use style::CLASS;
use tw_macro::assert_component;

/// The framed toolbar glyph in a legend row. A leaf: the row passes which icon
/// to draw.
#[component]
pub fn HelpLegendIcon(props: HelpLegendIconProps) -> Element {
    let icon = props.icon;
    rsx! {
        span { class: CLASS, aria_hidden: "true", dangerous_inner_html: icon }
    }
}

assert_component!(HelpLegendIcon);
