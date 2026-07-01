mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use style::CLASS;

pub use props::GridHeadingProps;

assert_component!(GridHeading);

#[component]
pub fn GridHeading(props: GridHeadingProps) -> Element {
    let heading = props.heading;
    rsx! {
        h3 {
            class: CLASS,
            {heading}
        }
    }
}
