pub mod components;
mod model;
mod presentation;
mod view;

pub use view::UnitStatsPanelView;
pub mod stat_icon;
mod style;

use components::attributes_column::AttributesColumn;
use components::combat_column::CombatColumn;
use components::defense_column::DefenseColumn;
use components::vitality_column::VitalityColumn;
use dioxus::prelude::*;
use model::UnitStatsPanelModel;
use presentation::{UnitStatsPanelPresentation, use_unit_stats_panel};
use style::CLASS;
use tw_macro::assert_component;

/// The four stat-category columns (vitality, combat, defense, attributes) in a 2×2
/// grid. Every figure is resolved through the domain in the composed hook and shaped
/// into each column's props; the body only places the columns. Combat and attributes
/// are guarded columns that render nothing when the unit has no attack or is not a
/// hero.
#[component]
pub fn UnitStatsPanel(props: UnitStatsPanelModel) -> Element {
    let UnitStatsPanelPresentation {
        hit_points,
        hit_points_regen,
        mana,
        mana_regen,
        attack,
        armor,
        defense_type,
        effective_hit_points,
        evasion,
        hero,
    } = use_unit_stats_panel(&props);
    rsx! {
        div {
            class: CLASS,
            VitalityColumn { hit_points, hit_points_regen, mana, mana_regen }
            CombatColumn { attack }
            DefenseColumn { armor, defense_type, effective_hit_points, evasion }
            AttributesColumn { hero }
        }
    }
}

assert_component!(UnitStatsPanel);
