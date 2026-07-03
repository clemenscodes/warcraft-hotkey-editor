pub mod components;
mod hooks;
mod props;
pub mod stat_icon;
mod style;

use crate::assert_component;
use components::attributes_column::AttributesColumn;
use components::combat_column::CombatColumn;
use components::defense_column::DefenseColumn;
use components::vitality_column::VitalityColumn;
use dioxus::prelude::*;
use hooks::{UnitStatsPanelModel, use_unit_stats_panel};
pub use props::UnitStatsPanelProps;
use style::CLASS;
assert_component!(UnitStatsPanel);

/// The four stat-category columns (vitality, combat, defense, attributes) in a 2×2
/// grid. Every figure is resolved through the domain in the composed hook and shaped
/// into each column's props; the body only places the columns. Combat and attributes
/// are guarded columns that render nothing when the unit has no attack or is not a
/// hero.
#[component]
pub fn UnitStatsPanel(props: UnitStatsPanelProps) -> Element {
    let model = use_unit_stats_panel(&props);
    let UnitStatsPanelModel {
        vitality,
        combat,
        defense,
        attributes,
    } = model;
    rsx! {
        div {
            class: CLASS,
            VitalityColumn { ..vitality }
            CombatColumn { ..combat }
            DefenseColumn { ..defense }
            AttributesColumn { ..attributes }
        }
    }
}
