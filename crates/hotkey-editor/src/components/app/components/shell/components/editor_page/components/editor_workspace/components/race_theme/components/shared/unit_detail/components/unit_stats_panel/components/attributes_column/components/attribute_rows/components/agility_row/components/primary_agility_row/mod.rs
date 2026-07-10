mod props;
mod view;

pub use view::PrimaryAgilityRowView;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::attributes_column::components::attribute_rows::components::shared::primary_attribute_label::PrimaryAttributeLabel;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::stat_gain::StatGain;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::components::shared::stat_value::StatValue;
use dioxus::prelude::*;
use props::PrimaryAgilityRowProps;
use style::CLASS;
use tw_macro::assert_component;

/// The hero's agility row when agility is the hero's primary attribute: it wears the
/// gold primary glow and publishes the full-gold `--attribute-label-color` its label reads.
#[component]
pub fn PrimaryAgilityRow(props: PrimaryAgilityRowProps) -> Element {
    let statistic = props.statistic;
    let growth = props.growth;
    let label = props.label;
    rsx! {
        div {
            class: CLASS,
            PrimaryAttributeLabel { text: label }
            StatValue { value: statistic }
            StatGain { value: growth }
        }
    }
}

assert_component!(PrimaryAgilityRow);
