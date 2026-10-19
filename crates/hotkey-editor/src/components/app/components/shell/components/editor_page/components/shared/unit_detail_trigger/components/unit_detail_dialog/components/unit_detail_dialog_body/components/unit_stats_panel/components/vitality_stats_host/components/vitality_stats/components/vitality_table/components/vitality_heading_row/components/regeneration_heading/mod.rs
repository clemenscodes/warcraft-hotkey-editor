mod model;
mod view;

mod style;

use dioxus::prelude::*;
use model::RegenerationHeadingModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn RegenerationHeading(props: RegenerationHeadingModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(RegenerationHeading);
