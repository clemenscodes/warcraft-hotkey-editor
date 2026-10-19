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
    let mark = props.mark;
    let condition = props.condition;
    rsx! {
        span {
            class: CLASS,
            title: condition,
            {text}
            if let Some(icon) = mark {
                span {
                    aria_hidden: "true",
                    dangerous_inner_html: icon,
                }
            }
        }
    }
}

assert_component!(MutedHitPointsRegenGain);
