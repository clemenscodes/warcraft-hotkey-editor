mod props;
mod style;

use super::super::super::shared::stat_value::StatValue;
use dioxus::prelude::*;
pub use props::DamagePerSecondRowProps;
use style::{CLASS, LABEL};
use tw_macro::assert_component;
assert_component!(DamagePerSecondRow);

const LABEL_TEXT: &str = "Damage per Second";

/// The damage-per-second row, shown only when the attack has a real cooldown. A
/// guarded leaf that early-returns when the rate is undefined.
#[component]
pub fn DamagePerSecondRow(props: DamagePerSecondRowProps) -> Element {
    let Some(value) = props.value else {
        return rsx! {};
    };
    rsx! {
        div {
            class: CLASS,
            span { class: LABEL, {LABEL_TEXT} }
            StatValue { value }
        }
    }
}
