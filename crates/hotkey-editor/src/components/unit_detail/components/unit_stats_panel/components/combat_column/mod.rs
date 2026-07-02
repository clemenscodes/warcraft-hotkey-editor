pub mod components;
mod data;
mod logic;
mod props;

use super::shared::stat_column::{StatColumn, StatColumnKind};
use super::shared::stat_icon_frame::{StatIconFrame, StatIconFrameProps};
use super::shared::stat_row::StatRow;
use super::shared::stat_rows::StatRows;
use components::damage_matchup_row::DamageMatchupRow;
use dioxus::prelude::*;
pub use props::{AttackDisplayData, CombatColumnProps};

/// The combat column: the attack-type icon beside the damage/range/speed rows and
/// the damage matchup grid. Present only when the unit has an attack.
#[component]
pub fn CombatColumn(props: CombatColumnProps) -> Element {
    let Some(attack) = props.attack else {
        return rsx! {};
    };
    let icon = StatIconFrameProps::from(&attack);
    let attack_type = attack.attack_type();
    let lines = logic::combat_lines(&attack);
    rsx! {
        StatColumn {
            kind: StatColumnKind::Combat,
            with_icon: true,
            StatIconFrame { ..icon }
            StatRows {
                for line in lines {
                    StatRow {
                        label: line.label,
                        value: Some(line.value),
                    }
                }
                DamageMatchupRow { attack_type }
            }
        }
    }
}
