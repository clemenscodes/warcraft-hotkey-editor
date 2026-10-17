mod model;
mod view;

mod style;

use dioxus::prelude::*;
use model::MutedHitPointsRegenGainModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn MutedHitPointsRegenGain(props: MutedHitPointsRegenGainModel) -> Element {
    let text = props.text;
    let condition = props.condition;
    rsx! {
        span {
            class: CLASS,
            title: condition,
            {text}
        }
    }
}

assert_component!(MutedHitPointsRegenGain);
