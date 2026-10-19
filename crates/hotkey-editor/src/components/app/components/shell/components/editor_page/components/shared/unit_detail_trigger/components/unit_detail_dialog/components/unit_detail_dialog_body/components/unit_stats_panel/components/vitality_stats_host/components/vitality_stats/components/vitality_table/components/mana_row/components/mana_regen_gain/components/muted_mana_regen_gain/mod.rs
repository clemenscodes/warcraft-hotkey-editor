mod model;
mod view;

mod style;

use dioxus::prelude::*;
use model::MutedManaRegenGainModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn MutedManaRegenGain(props: MutedManaRegenGainModel) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(MutedManaRegenGain);
