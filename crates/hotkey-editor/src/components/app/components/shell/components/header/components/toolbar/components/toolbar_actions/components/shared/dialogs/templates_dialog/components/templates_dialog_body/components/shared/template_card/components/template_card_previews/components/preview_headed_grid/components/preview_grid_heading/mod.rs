mod model;
mod view;

pub use view::PreviewGridHeadingView;
mod style;

use dioxus::prelude::*;
use model::PreviewGridHeadingModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PreviewGridHeading(props: PreviewGridHeadingModel) -> Element {
    let heading = props.heading;
    rsx! {
        h3 {
            class: CLASS,
            {heading}
        }
    }
}

assert_component!(PreviewGridHeading);
