mod props;
mod style;

use super::super::super::shared::stat_value::StatValue;
use dioxus::prelude::*;
pub use props::DefenseTypeRowProps;
use style::{CLASS, LABEL};
use tw_macro::assert_component;
assert_component!(DefenseTypeRow);

const LABEL_TEXT: &str = "Defense Type";

/// The unit's defense type — what governs how incoming damage is scaled.
#[component]
pub fn DefenseTypeRow(props: DefenseTypeRowProps) -> Element {
    let value = props.value;
    rsx! {
        div {
            class: CLASS,
            span { class: LABEL, {LABEL_TEXT} }
            StatValue { value }
        }
    }
}
