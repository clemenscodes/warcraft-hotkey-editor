mod model;
mod view;

mod style;

use dioxus::prelude::*;
use model::ActiveManaRegenGainModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ActiveManaRegenGain(props: ActiveManaRegenGainModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(ActiveManaRegenGain);
