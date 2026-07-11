mod model;
mod view;

pub use view::GridHeadingView;
mod style;

use dioxus::prelude::*;
use model::GridHeadingModel;
use style::CLASS;
use tw_macro::assert_component;

/// A grid section heading: an `h3` wearing the uppercase gold heading look, with
/// its own per-band sizing.
#[component]
pub fn GridHeading(props: GridHeadingModel) -> Element {
    let heading = props.heading;
    rsx! {
        h3 {
            class: CLASS,
            {heading}
        }
    }
}

assert_component!(GridHeading);
