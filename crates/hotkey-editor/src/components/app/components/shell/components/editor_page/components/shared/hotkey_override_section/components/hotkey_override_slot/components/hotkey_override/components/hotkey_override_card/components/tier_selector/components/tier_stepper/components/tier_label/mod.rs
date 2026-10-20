mod model;
mod style;
mod view;

pub use view::TierLabelView;

use dioxus::prelude::*;
use model::TierLabelModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn TierLabel(props: TierLabelModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(TierLabel);
