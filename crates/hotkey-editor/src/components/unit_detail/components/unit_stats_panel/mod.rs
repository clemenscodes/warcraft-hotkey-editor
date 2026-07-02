pub mod components;
mod hooks;
mod leveled_stats;
mod logic;
mod props;
pub mod stat_icon;
mod style;

use crate::assert_component;
use components::attributes_column::AttributesColumn;
use components::combat_column::CombatColumn;
use components::defense_matchup_row::DefenseMatchupRow;
use components::shared::stat_column::{StatColumn, StatColumnKind};
use components::shared::stat_icon_frame::{StatIconFrame, StatIconFrameProps};
use components::shared::stat_row::StatRow;
use components::shared::stat_rows::StatRows;
use dioxus::prelude::*;
use hooks::use_unit_stats_panel;
pub use props::UnitStatsPanelProps;
use style::CLASS;
assert_component!(UnitStatsPanel);

/// The four stat-category columns (vitality, combat, defense, attributes) in a 2×2
/// grid. Every figure is resolved in the composed hook and each column's rows are
/// shaped into a list; the body only places the columns and loops their rows.
#[component]
pub fn UnitStatsPanel(props: UnitStatsPanelProps) -> Element {
    let model = use_unit_stats_panel(&props);
    let vitality_rows = logic::vitality_rows(&model);
    let defense_rows = logic::defense_rows(&model);
    let defense_type = model.defense_type;
    let src = model.defense_icon;
    let alt = model.defense_icon_alt;
    let defense_icon = StatIconFrameProps { src, alt };
    let attack = model.attack;
    let hero = model.hero;
    rsx! {
        div {
            class: CLASS,
            StatColumn {
                kind: StatColumnKind::Vitality,
                for row in vitality_rows {
                    StatRow { ..row }
                }
            }
            CombatColumn { attack }
            StatColumn {
                kind: StatColumnKind::Defense,
                with_icon: true,
                StatIconFrame { ..defense_icon }
                StatRows {
                    for row in defense_rows {
                        StatRow { ..row }
                    }
                    DefenseMatchupRow { defense_type }
                }
            }
            AttributesColumn { hero }
        }
    }
}
