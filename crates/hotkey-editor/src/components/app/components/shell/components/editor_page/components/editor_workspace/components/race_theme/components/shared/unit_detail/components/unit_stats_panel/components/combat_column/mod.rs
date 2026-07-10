pub mod components;
mod logic;
mod props;
mod style;

use super::shared::stat_icon_frame::StatIconFrame;
use components::combat_rows::{CombatRows, CombatRowsProps};
use dioxus::prelude::*;
use logic::CombatFigures;
pub use props::CombatColumnProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(CombatColumn);

/// The combat column: the attack-type icon beside its stat rows and matchup grid, laid
/// into the `combat` grid area. Present only when the unit has an attack; a unit that
/// cannot attack renders nothing here.
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
    let rows = CombatRowsProps {
        damage,
        range,
        speed,
        damage_per_second,
        attack_type,
    };
    rsx! {
        div {
            class: CLASS,
            StatIconFrame { ..icon }
            CombatRows { ..rows }
        }
    }
}
