mod components;
mod logic;
mod props;
mod style;

use super::super::super::shared::regen_qualifier::RegenQualifier;
use super::shared::regen_label::RegenLabel;
use components::hit_points_regen_gain::HitPointsRegenGain;
use dioxus::prelude::*;
use logic::HitPointsRegenPresentation;
pub use props::HitPointsRegenRowProps;
use style::CLASS;
use tw_macro::assert_component;

const LABEL_TEXT: &str = "Regeneration";

/// The unit's health regeneration: an indented companion to the hit points row. It
/// carries its own conditional qualifier ("at night", "on blight") and its green gain,
/// dimmed when the unit does not regenerate.
#[component]
pub fn HitPointsRegenRow(props: HitPointsRegenRowProps) -> Element {
    let value = props.value;
    let HitPointsRegenPresentation { qualifier } = HitPointsRegenPresentation::from(value);
    rsx! {
        div {
            class: CLASS,
            RegenLabel { text: LABEL_TEXT }
            RegenQualifier { text: qualifier }
            HitPointsRegenGain { value }
        }
    }
}

assert_component!(HitPointsRegenRow);
