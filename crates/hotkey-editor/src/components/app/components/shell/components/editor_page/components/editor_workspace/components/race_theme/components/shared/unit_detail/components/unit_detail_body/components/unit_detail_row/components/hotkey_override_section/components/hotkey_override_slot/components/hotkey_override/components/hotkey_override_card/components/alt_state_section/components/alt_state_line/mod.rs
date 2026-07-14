mod model;
mod view;

pub use view::AltStateLineView;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use model::AltStateLineModel;

#[component]
pub fn AltStateLine(props: AltStateLineModel) -> Element {
    let text = props.text;
    rsx! {
        p {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(AltStateLine);
