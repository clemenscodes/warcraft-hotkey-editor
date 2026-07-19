mod components;
mod model;
mod presentation;
mod view;

pub use view::HitPointsRegenRowView;
mod style;

use super::super::super::super::super::shared::regen_qualifier::RegenQualifier;
use super::shared::regen_label::RegenLabel;
use components::hit_points_regen_gain::HitPointsRegenGain;
use dioxus::prelude::*;
use model::HitPointsRegenRowModel;
use presentation::HitPointsRegenPresentation;
use style::CLASS;
use tw_macro::assert_component;

const LABEL_TEXT: &str = "Regeneration";

#[component]
pub fn HitPointsRegenRow(props: HitPointsRegenRowModel) -> Element {
    let value = props.value;
    let HitPointsRegenPresentation { qualifier } = HitPointsRegenPresentation::from(value);
    rsx! {
        div {
            class: CLASS,
            RegenLabel {
                text: LABEL_TEXT,
            }
            RegenQualifier {
                text: qualifier,
            }
            HitPointsRegenGain {
                value,
            }
        }
    }
}

assert_component!(HitPointsRegenRow);
