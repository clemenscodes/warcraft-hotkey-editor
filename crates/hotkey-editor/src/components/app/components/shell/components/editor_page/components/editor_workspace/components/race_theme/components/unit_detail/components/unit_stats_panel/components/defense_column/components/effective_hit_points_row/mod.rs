mod props;
mod style;

use super::super::super::shared::stat_value::StatValue;
use dioxus::prelude::*;
pub use props::EffectiveHitPointsRowProps;
use style::{CLASS, LABEL};
use tw_macro::assert_component;
assert_component!(EffectiveHitPointsRow);

const LABEL_TEXT: &str = "Effective Hit Points";

/// The unit's effective hit points — raw health scaled by armor mitigation.
#[component]
pub fn EffectiveHitPointsRow(props: EffectiveHitPointsRowProps) -> Element {
    let value = props.value;
    rsx! {
        div {
            class: CLASS,
            span { class: LABEL, {LABEL_TEXT} }
            StatValue { value }
        }
    }
}
