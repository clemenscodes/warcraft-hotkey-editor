mod model;
mod view;

pub use view::ActiveStatValueView;
mod style;

use dioxus::prelude::*;
use model::ActiveStatValueModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ActiveStatValue(props: ActiveStatValueModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(ActiveStatValue);
