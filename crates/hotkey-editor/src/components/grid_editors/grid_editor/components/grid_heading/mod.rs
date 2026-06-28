mod props;
mod style;

use dioxus::prelude::*;

use style::GRID_HEADING_STYLE_SHEETS;

pub use props::GridHeadingProps;

#[component]
pub fn GridHeading(props: GridHeadingProps) -> Element {
    rsx! {
        for href in GRID_HEADING_STYLE_SHEETS {
            document::Stylesheet { href }
        }
        h3 {
            class: "grid-heading",
            { props.heading }
        }
    }
}
