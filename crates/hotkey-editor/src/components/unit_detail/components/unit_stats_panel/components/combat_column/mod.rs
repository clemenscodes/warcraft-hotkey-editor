pub mod components;
mod props;

use super::stat_column::{StatColumn, StatColumnKind};
use super::stat_icon_frame::{StatIconFrame, StatIconFrameProps};
use super::stat_row::StatRow;
use super::stat_row_label::StatRowLabel;
use super::stat_row_value::StatRowValue;
use super::stat_rows::StatRows;
use components::damage_matchup_row::DamageMatchupRow;
use dioxus::prelude::*;
pub use props::{AttackDisplayData, CombatColumnProps};

/// The combat column: the attack-type icon beside the damage/range/speed rows and
/// the damage matchup grid.
#[component]
pub fn CombatColumn(props: CombatColumnProps) -> Element {
    let attack = props.attack;
    let icon = StatIconFrameProps::from(&attack);
    let damage_text = attack.damage_text().to_owned();
    let attack_range = attack.attack_range();
    let attack_range_text = attack_range.to_string();
    let speed_text = attack.speed_text().to_owned();
    let damage_per_second_text = attack.damage_per_second_text().map(str::to_owned);
    let type_label = attack.type_label().to_owned();
    let attack_type = attack.attack_type();
    rsx! {
        StatColumn {
            kind: StatColumnKind::Combat,
            with_icon: true,
            StatIconFrame { ..icon }
            StatRows {
                StatRow {
                    StatRowLabel { text: "Damage" }
                    StatRowValue { text: damage_text }
                }
                if attack_range > 0 {
                    StatRow {
                        StatRowLabel { text: "Range" }
                        StatRowValue { text: attack_range_text }
                    }
                }
                StatRow {
                    StatRowLabel { text: "Attack Speed" }
                    StatRowValue { text: speed_text }
                }
                if let Some(dps_text) = damage_per_second_text {
                    StatRow {
                        StatRowLabel { text: "Damage per Second" }
                        StatRowValue { text: dps_text }
                    }
                }
                StatRow {
                    StatRowLabel { text: "Attack Type" }
                    StatRowValue { text: type_label }
                }
                DamageMatchupRow { attack_type }
            }
        }
    }
}
