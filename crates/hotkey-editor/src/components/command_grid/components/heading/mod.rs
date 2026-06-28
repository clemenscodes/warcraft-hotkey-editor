mod props;
mod style;

use dioxus::prelude::*;

use props::CommandGridHeadingProps;
use style::COMMAND_GRID_HEADING_STYLE_SHEETS;

#[component]
pub fn CommandGridHeading(props: CommandGridHeadingProps) -> Element {
    rsx! {
        for style_sheet in COMMAND_GRID_HEADING_STYLE_SHEETS {
            document::Stylesheet { href: style_sheet }
        }
        h3 { class: "command-section-heading", {props.heading} }
    }
}
