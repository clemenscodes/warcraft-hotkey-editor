pub mod components;
mod logic;
mod props;
mod style;

use super::shared::stat_icon_frame::StatIconFrame;
use components::attack_speed_row::AttackSpeedRow;
use components::attack_type_row::AttackTypeRow;
use components::damage_matchup_row::DamageMatchupRow;
use components::damage_per_second_row::DamagePerSecondRow;
use components::damage_row::DamageRow;
use components::range_row::RangeRow;
use dioxus::prelude::*;
use logic::CombatFigures;
pub use props::CombatColumnProps;
use style::{CLASS, ROWS};
use tw_macro::assert_component;
assert_component!(CombatColumn);

/// The combat column: the attack-type icon beside the damage/range/speed rows and the
/// damage matchup grid, laid into the `combat` grid area. Present only when the unit
/// has an attack; a unit that cannot attack renders nothing here. It names its rows
/// directly — each row owns its own look.
#[component]
pub fn CombatColumn(props: CombatColumnProps) -> Element {
    let Some(attack) = props.attack else {
        return rsx! {};
    };
    let CombatFigures {
        icon,
        damage,
        range,
        speed,
        damage_per_second,
        attack_type,
    } = CombatFigures::from(&attack);
    rsx! {
        div {
            class: CLASS,
            StatIconFrame { ..icon }
            div {
                class: ROWS,
                DamageRow { value: damage }
                RangeRow { value: range }
                AttackSpeedRow { value: speed }
                DamagePerSecondRow { value: damage_per_second }
                AttackTypeRow { value: attack_type }
                DamageMatchupRow { attack_type }
            }
        }
    }
}
