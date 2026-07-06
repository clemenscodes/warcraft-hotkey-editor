mod props;
mod style;

use dioxus::prelude::*;
pub use props::GridHeadingProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(GridHeading);

#[component]
pub fn GridHeading(props: GridHeadingProps) -> Element {
    let heading = props.heading;
    rsx! {
        h3 { class: CLASS, {heading} }
    }
}
