mod model;
mod view;

pub use view::AbilityTierLabelView;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use model::AbilityTierLabelModel;

#[component]
pub fn AbilityTierLabel(props: AbilityTierLabelModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(AbilityTierLabel);
