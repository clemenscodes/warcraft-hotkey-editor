mod model;
mod view;

pub use view::SwitchItemLabelView;
mod style;

use dioxus::prelude::*;
use model::SwitchItemLabelModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SwitchItemLabel(props: SwitchItemLabelModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(SwitchItemLabel);
