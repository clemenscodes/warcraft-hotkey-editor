pub mod components;
mod data;
mod kinds;
mod logic;
mod props;

use super::shared::stat_column::{StatColumn, StatColumnKind};
use super::shared::stat_icon_frame::StatIconFrame;
use super::shared::stat_row::StatRow;
use super::shared::stat_rows::StatRows;
use components::damage_matchup_row::DamageMatchupRow;
use components::damage_per_second_row::DamagePerSecondRow;
use components::range_row::RangeRow;
use dioxus::prelude::*;
use logic::CombatRows;
pub use props::CombatColumnProps;

/// The combat column: the attack-type icon beside the damage/range/speed rows and the
/// damage matchup grid. Present only when the unit has an attack; a unit that cannot
/// attack renders nothing here.
#[component]
pub fn CombatColumn(props: CombatColumnProps) -> Element {
    let Some(attack) = props.attack else {
        return rsx! {};
    };
    let CombatRows {
        icon,
        damage_row,
        range,
        speed_row,
        damage_per_second,
        attack_type_row,
        attack_type,
    } = CombatRows::from(&attack);
    rsx! {
        StatColumn {
            kind: StatColumnKind::Combat,
            with_icon: true,
            StatIconFrame { ..icon }
            StatRows {
                StatRow { ..damage_row }
                RangeRow { range }
                StatRow { ..speed_row }
                DamagePerSecondRow { damage_per_second }
                StatRow { ..attack_type_row }
                DamageMatchupRow { attack_type }
            }
        }
    }
}
