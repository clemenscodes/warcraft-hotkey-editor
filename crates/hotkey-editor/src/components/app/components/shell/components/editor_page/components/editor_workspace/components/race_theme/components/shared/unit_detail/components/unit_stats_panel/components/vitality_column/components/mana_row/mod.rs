mod components;
mod props;
mod style;

use super::super::super::shared::stat_label::StatLabel;
use components::mana_value::ManaValue;
use dioxus::prelude::*;
pub use props::ManaRowProps;
use style::CLASS;
use tw_macro::assert_component;

const LABEL_TEXT: &str = "Mana";

/// The unit's mana pool. Wears the human-blue accent directly; a unit with no mana
/// reports itself muted and the figure dims to faint.
#[component]
pub fn ManaRow(props: ManaRowProps) -> Element {
    let value = props.value;
    rsx! {
        div {
            class: CLASS,
            StatLabel { text: LABEL_TEXT }
            ManaValue { value }
        }
    }
}

assert_component!(ManaRow);
