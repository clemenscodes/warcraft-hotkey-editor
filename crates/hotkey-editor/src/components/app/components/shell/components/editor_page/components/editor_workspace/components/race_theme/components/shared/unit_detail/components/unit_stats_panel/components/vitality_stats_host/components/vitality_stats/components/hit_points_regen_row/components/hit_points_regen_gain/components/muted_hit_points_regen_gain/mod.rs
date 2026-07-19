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
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(MutedHitPointsRegenGain);
