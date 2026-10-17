mod model;
mod view;

mod style;

use dioxus::prelude::*;
use model::ActiveHitPointsRegenGainModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ActiveHitPointsRegenGain(props: ActiveHitPointsRegenGainModel) -> Element {
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

assert_component!(ActiveHitPointsRegenGain);
