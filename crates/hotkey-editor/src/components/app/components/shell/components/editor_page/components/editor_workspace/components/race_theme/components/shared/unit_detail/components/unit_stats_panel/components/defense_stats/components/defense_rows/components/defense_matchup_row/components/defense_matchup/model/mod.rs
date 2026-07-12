use super::view::DefenseMatchupView;
use dioxus::prelude::*;
use warcraft_api::{AttackType, DefenseType};

/// One cell of a defender's matchup: how an attack type fares against the defense.
#[derive(Props, Clone, PartialEq)]
pub struct DefenseMatchupModel {
    pub attack_type: AttackType,
    pub defense_type: DefenseType,
}

impl From<&DefenseMatchupView> for DefenseMatchupModel {
    fn from(view: &DefenseMatchupView) -> Self {
        let DefenseMatchupView {
            attack_type,
            defense_type,
        } = view.clone();
        Self {
            attack_type,
            defense_type,
        }
    }
}

impl ddd::Model for DefenseMatchupModel {
    type View = DefenseMatchupView;
}
