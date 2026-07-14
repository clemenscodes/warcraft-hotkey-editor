mod model;
mod view;

mod style;

use dioxus::prelude::*;
use model::MutedManaValueModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn MutedManaValue(props: MutedManaValueModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(MutedManaValue);
