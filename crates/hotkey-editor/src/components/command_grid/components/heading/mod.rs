mod props;
mod style;

use dioxus::prelude::*;

use props::CommandGridHeadingProps;
use style::COMMAND_GRID_HEADING_STYLE_SHEETS;

#[component]
pub fn CommandGridHeading(props: CommandGridHeadingProps) -> Element {
    rsx! {
        for href in COMMAND_GRID_HEADING_STYLE_SHEETS {
            document::Stylesheet { href }
        }
        h3 {
            class: "command-grid-heading",
            { props.heading }
        }
    }
}
