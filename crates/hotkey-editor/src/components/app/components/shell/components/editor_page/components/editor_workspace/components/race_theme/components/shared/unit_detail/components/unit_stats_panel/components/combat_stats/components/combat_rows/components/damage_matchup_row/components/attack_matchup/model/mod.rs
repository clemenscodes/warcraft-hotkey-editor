use super::view::AttackMatchupView;
use dioxus::prelude::*;
use warcraft_api::{AttackType, DefenseType};

/// One cell of an attacker's damage matchup: how the attack fares against a defense.
#[derive(Props, Clone, PartialEq)]
pub struct AttackMatchupModel {
    pub defense_type: DefenseType,
    pub attack_type: AttackType,
}

impl From<&AttackMatchupView> for AttackMatchupModel {
    fn from(view: &AttackMatchupView) -> Self {
        let AttackMatchupView {
            defense_type,
            attack_type,
        } = view.clone();
        Self {
            defense_type,
            attack_type,
        }
    }
}

impl ddd::Model for AttackMatchupModel {
    type View = AttackMatchupView;
}
