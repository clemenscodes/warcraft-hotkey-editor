pub mod components;
mod model;
mod view;

pub use view::CombatRowsView;
mod style;

use components::attack_speed_row::AttackSpeedRow;
use components::attack_type_row::AttackTypeRow;
use components::damage_matchup_row::DamageMatchupRow;
use components::damage_per_second_row::DamagePerSecondRow;
use components::damage_row::DamageRow;
use components::range_row::RangeRow;
use dioxus::prelude::*;
use model::CombatRowsModel;
use style::CLASS;
use tw_macro::assert_component;

/// The combat column's stat rows stacked beside its icon: the damage, range, speed, and
/// damage-per-second rows, the attack-type row, and the damage matchup grid. It names
/// its rows directly — each row owns its own look.
#[component]
pub fn CombatRows(props: CombatRowsModel) -> Element {
    let damage = props.damage;
    let range = props.range;
    let speed = props.speed;
    let damage_per_second = props.damage_per_second;
    let attack_type = props.attack_type;
    rsx! {
        div {
            class: CLASS,
            DamageRow { value: damage }
            RangeRow { value: range }
            AttackSpeedRow { value: speed }
            DamagePerSecondRow { value: damage_per_second }
            AttackTypeRow { value: attack_type }
            DamageMatchupRow { attack_type }
        }
    }
}

assert_component!(CombatRows);
