mod components;
mod model;
mod view;

pub use view::ManaRowView;
mod style;

use super::super::super::super::super::super::super::shared::stat_label::StatLabel;
use components::mana_regen_gain::ManaRegenGain;
use components::mana_value::ManaValue;
use dioxus::prelude::*;
use model::ManaRowModel;
use style::CLASS;
use tw_macro::assert_component;

const LABEL_TEXT: &str = "Mana";

#[component]
pub fn ManaRow(props: ManaRowModel) -> Element {
    let value = props.value;
    let regen = props.regen;
    rsx! {
        div {
            class: CLASS,
            StatLabel {
                text: LABEL_TEXT,
            }
            ManaValue {
                value,
            }
            ManaRegenGain {
                value: regen,
            }
        }
    }
}

assert_component!(ManaRow);
