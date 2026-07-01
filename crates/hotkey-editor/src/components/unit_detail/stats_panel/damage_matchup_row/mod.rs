use super::attack_matchup_cell::AttackMatchupCell;
use dioxus::prelude::*;
use warcraft_api::{AttackType, DefenseType};

const DISPLAYED_DEFENSE_TYPES: [DefenseType; 7] = [
    DefenseType::Light,
    DefenseType::Medium,
    DefenseType::Heavy,
    DefenseType::Fortified,
    DefenseType::Hero,
    DefenseType::Divine,
    DefenseType::Unarmored,
];

#[derive(Props, Clone, PartialEq)]
pub struct DamageMatchupRowProps {
    pub attack_type: AttackType,
}

#[component]
pub fn DamageMatchupRow(props: DamageMatchupRowProps) -> Element {
    let attack_type = props.attack_type;
    rsx! {
        div { class: "damage-matchup",
            for defense_type in DISPLAYED_DEFENSE_TYPES {
                AttackMatchupCell { defense_type, attack_type }
            }
        }
    }
}
