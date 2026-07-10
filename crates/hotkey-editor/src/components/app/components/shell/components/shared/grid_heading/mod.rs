mod props;
mod style;

use dioxus::prelude::*;
use props::GridHeadingProps;
use style::CLASS;
use tw_macro::assert_component;

/// A grid section heading: an `h3` wearing the uppercase gold heading look, with
/// its own per-band sizing.
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

assert_component!(GridHeading);
