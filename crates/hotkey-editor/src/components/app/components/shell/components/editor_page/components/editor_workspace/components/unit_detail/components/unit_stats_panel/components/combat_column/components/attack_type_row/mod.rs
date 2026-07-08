mod props;
mod style;

use super::super::super::shared::stat_value::StatValue;
use dioxus::prelude::*;
pub use props::AttackTypeRowProps;
use style::{CLASS, LABEL};
use tw_macro::assert_component;
assert_component!(AttackTypeRow);

const LABEL_TEXT: &str = "Attack Type";

/// The unit's attack type — what its damage is classified as.
#[component]
pub fn AttackTypeRow(props: AttackTypeRowProps) -> Element {
    let value = props.value;
    rsx! {
        div {
            class: CLASS,
            span { class: LABEL, {LABEL_TEXT} }
            StatValue { value }
        }
    }
}
