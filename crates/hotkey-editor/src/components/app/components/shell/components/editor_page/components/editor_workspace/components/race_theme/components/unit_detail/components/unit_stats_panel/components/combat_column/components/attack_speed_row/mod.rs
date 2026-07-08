mod props;
mod style;

use super::super::super::shared::stat_value::StatValue;
use dioxus::prelude::*;
pub use props::AttackSpeedRowProps;
use style::{CLASS, LABEL};
use tw_macro::assert_component;
assert_component!(AttackSpeedRow);

const LABEL_TEXT: &str = "Attack Speed";

/// The unit's attack cooldown, shown in seconds.
#[component]
pub fn AttackSpeedRow(props: AttackSpeedRowProps) -> Element {
    let value = props.value;
    rsx! {
        div {
            class: CLASS,
            span { class: LABEL, {LABEL_TEXT} }
            StatValue { value }
        }
    }
}
