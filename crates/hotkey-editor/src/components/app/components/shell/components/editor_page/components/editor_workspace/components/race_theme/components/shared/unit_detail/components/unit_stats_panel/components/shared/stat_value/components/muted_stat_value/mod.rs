mod model;
mod view;

pub use view::MutedStatValueView;
mod style;

use dioxus::prelude::*;
use model::MutedStatValueModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn MutedStatValue(props: MutedStatValueModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(MutedStatValue);
