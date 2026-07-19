mod model;
mod view;

pub use view::PrimaryAttributeLabelView;
mod style;

use dioxus::prelude::*;
use model::PrimaryAttributeLabelModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PrimaryAttributeLabel(props: PrimaryAttributeLabelModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(PrimaryAttributeLabel);
