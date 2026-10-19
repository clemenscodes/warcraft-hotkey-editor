mod model;
mod view;

mod style;

use dioxus::prelude::*;
use model::ActiveManaValueModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ActiveManaValue(props: ActiveManaValueModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(ActiveManaValue);
