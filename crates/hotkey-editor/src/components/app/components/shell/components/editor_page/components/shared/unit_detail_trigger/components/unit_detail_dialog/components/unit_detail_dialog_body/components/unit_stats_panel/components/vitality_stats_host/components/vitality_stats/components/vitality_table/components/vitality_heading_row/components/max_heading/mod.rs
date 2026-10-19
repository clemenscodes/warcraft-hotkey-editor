mod model;
mod view;

mod style;

use dioxus::prelude::*;
use model::MaxHeadingModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn MaxHeading(props: MaxHeadingModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(MaxHeading);
