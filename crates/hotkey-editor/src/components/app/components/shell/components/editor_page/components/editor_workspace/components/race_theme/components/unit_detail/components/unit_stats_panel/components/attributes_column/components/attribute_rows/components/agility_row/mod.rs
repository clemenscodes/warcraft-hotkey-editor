mod props;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_stats_panel::components::shared::stat_gain::StatGain;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_stats_panel::components::shared::stat_value::StatValue;
use super::shared::primary_attribute_label::PrimaryAttributeLabel;
use dioxus::prelude::*;
pub use props::AgilityRowProps;
use style::CLASS;
use tw_macro::assert_component;
use warcraft_api::PrimaryAttribute;
assert_component!(AgilityRow);

/// The hero's agility attribute: its value and per-level growth, wearing a gold glow
/// when agility is the hero's primary attribute.
#[component]
pub fn AgilityRow(props: AgilityRowProps) -> Element {
    let statistic = props.statistic;
    let growth = statistic.growth();
    let is_primary = props.is_primary;
    let attribute = PrimaryAttribute::Agility;
    let label = attribute.to_string();
    rsx! {
        div {
            class: CLASS,
            "data-primary": is_primary,
            PrimaryAttributeLabel { text: label }
            StatValue { value: statistic }
            StatGain { value: growth }
        }
    }
}
