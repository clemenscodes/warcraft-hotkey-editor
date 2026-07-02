pub mod components;
mod data;
mod hooks;
mod kinds;
mod props;
pub mod stat_icon;
mod style;

use crate::assert_component;
use components::attributes_column::AttributesColumn;
use components::combat_column::CombatColumn;
use components::defense_matchup_row::DefenseMatchupRow;
use components::evasion_row::EvasionRow;
use components::shared::stat_column::{StatColumn, StatColumnKind};
use components::shared::stat_icon_frame::StatIconFrame;
use components::shared::stat_row::StatRow;
use components::shared::stat_rows::StatRows;
use dioxus::prelude::*;
use hooks::{UnitStatsPanelModel, use_unit_stats_panel};
pub use props::UnitStatsPanelProps;
use style::CLASS;
assert_component!(UnitStatsPanel);

/// The four stat-category columns (vitality, combat, defense, attributes) in a 2×2
/// grid. Every figure is resolved through the domain in the composed hook and shaped
/// into row props; the body only places the columns and their rows. Vitality and
/// defense are laid out inline (the panel owns those columns); combat and attributes
/// are guarded child columns that render nothing when the unit has no attack or is
/// not a hero.
#[component]
pub fn UnitStatsPanel(props: UnitStatsPanelProps) -> Element {
    let model = use_unit_stats_panel(&props);
    let UnitStatsPanelModel {
        hit_points_row,
        hit_points_regen_row,
        mana_row,
        mana_regen_row,
        armor_row,
        defense_type_row,
        effective_hit_points_row,
        evasion,
        defense_type,
        defense_icon,
        combat,
        attributes,
    } = model;
    rsx! {
        div {
            class: CLASS,
            StatColumn {
                kind: StatColumnKind::Vitality,
                StatRow { ..hit_points_row }
                StatRow { ..hit_points_regen_row }
                StatRow { ..mana_row }
                StatRow { ..mana_regen_row }
            }
            CombatColumn { ..combat }
            StatColumn {
                kind: StatColumnKind::Defense,
                with_icon: true,
                StatIconFrame { ..defense_icon }
                StatRows {
                    StatRow { ..armor_row }
                    StatRow { ..defense_type_row }
                    StatRow { ..effective_hit_points_row }
                    EvasionRow { evasion }
                    DefenseMatchupRow { defense_type }
                }
            }
            AttributesColumn { ..attributes }
        }
    }
}
