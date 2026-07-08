mod props;
mod style;

use super::super::super::shared::stat_value::StatValue;
use dioxus::prelude::*;
pub use props::ArmorRowProps;
use style::{CLASS, LABEL};
use tw_macro::assert_component;
assert_component!(ArmorRow);

const LABEL_TEXT: &str = "Armor";

/// The unit's armor value.
#[component]
pub fn ArmorRow(props: ArmorRowProps) -> Element {
    let value = props.value;
    rsx! {
        div {
            class: CLASS,
            span { class: LABEL, {LABEL_TEXT} }
            StatValue { value }
        }
    }
}
