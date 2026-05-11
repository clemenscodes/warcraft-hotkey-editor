use dioxus::prelude::*;
use warcraft_api::{AttackType, DefenseType};

use super::defense_matchup_cell::DefenseMatchupCell;

const ALL_ATTACK_TYPES: [AttackType; 7] = [
    AttackType::Normal,
    AttackType::Pierce,
    AttackType::Siege,
    AttackType::Magic,
    AttackType::Chaos,
    AttackType::Hero,
    AttackType::Spells,
];

#[derive(Props, Clone, PartialEq)]
pub(super) struct DefenseMatchupRowProps {
    pub(super) defense_type: DefenseType,
}

#[component]
pub(super) fn DefenseMatchupRow(props: DefenseMatchupRowProps) -> Element {
    let defense_type = props.defense_type;
    rsx! {
        div { class: "damage-matchup",
            for attack_type in ALL_ATTACK_TYPES {
                DefenseMatchupCell { attack_type, defense_type }
            }
        }
    }
}
