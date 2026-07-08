mod props;
mod style;

use super::super::super::shared::stat_gain::StatGain;
use super::super::super::shared::stat_value::StatValue;
use dioxus::prelude::*;
pub use props::IntelligenceRowProps;
use style::{CLASS, LABEL};
use tw_macro::assert_component;
use warcraft_api::PrimaryAttribute;
assert_component!(IntelligenceRow);

/// The hero's intelligence attribute: its value and per-level growth, wearing a gold glow
/// when intelligence is the hero's primary attribute.
#[component]
pub fn IntelligenceRow(props: IntelligenceRowProps) -> Element {
    let statistic = props.statistic;
    let growth = statistic.growth();
    let is_primary = props.is_primary;
    let attribute = PrimaryAttribute::Intelligence;
    let label = attribute.to_string();
    rsx! {
        div {
            class: CLASS,
            "data-primary": is_primary,
            span { class: LABEL, {label} }
            StatValue { value: statistic }
            StatGain { value: growth }
        }
    }
}
