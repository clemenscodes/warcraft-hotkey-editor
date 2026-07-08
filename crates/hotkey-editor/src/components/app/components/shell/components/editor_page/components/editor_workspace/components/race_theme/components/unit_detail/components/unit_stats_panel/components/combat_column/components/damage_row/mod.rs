mod props;
mod style;

use super::super::super::shared::stat_value::StatValue;
use dioxus::prelude::*;
pub use props::DamageRowProps;
use style::{CLASS, LABEL};
use tw_macro::assert_component;
assert_component!(DamageRow);

const LABEL_TEXT: &str = "Damage";

/// The unit's attack damage range.
#[component]
pub fn DamageRow(props: DamageRowProps) -> Element {
    let value = props.value;
    rsx! {
        div {
            class: CLASS,
            span { class: LABEL, {LABEL_TEXT} }
            StatValue { value }
        }
    }
}
