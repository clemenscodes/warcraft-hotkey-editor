mod components;
mod props;
mod style;

use super::shared::regen_label::RegenLabel;
use components::mana_regen_gain::ManaRegenGain;
use dioxus::prelude::*;
pub use props::ManaRegenRowProps;
use style::CLASS;
use tw_macro::assert_component;

const LABEL_TEXT: &str = "Regeneration";

/// The unit's mana regeneration: an indented companion to the mana row, wearing the
/// human-blue accent. Dimmed when the unit does not regenerate mana.
#[component]
pub fn ManaRegenRow(props: ManaRegenRowProps) -> Element {
    let value = props.value;
    rsx! {
        div {
            class: CLASS,
            RegenLabel { text: LABEL_TEXT }
            ManaRegenGain { value }
        }
    }
}

assert_component!(ManaRegenRow);
