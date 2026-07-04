mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::GridHeadingProps;
use style::CLASS;
assert_component!(GridHeading);

#[component]
pub fn GridHeading(props: GridHeadingProps) -> Element {
    let heading = props.heading;
    rsx! {
        h3 { class: CLASS, {heading} }
    }
}
