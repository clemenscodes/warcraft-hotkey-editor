mod model;
mod view;

pub use view::GridHeadingView;
mod style;

use dioxus::prelude::*;
use model::GridHeadingModel;
use style::CLASS;
use tw_macro::assert_component;

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
