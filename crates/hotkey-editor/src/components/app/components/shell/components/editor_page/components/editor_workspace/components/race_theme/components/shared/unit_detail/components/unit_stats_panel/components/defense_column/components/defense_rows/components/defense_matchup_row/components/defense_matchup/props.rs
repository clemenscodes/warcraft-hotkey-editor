use dioxus::prelude::*;
use warcraft_api::{AttackType, DefenseType};

/// One cell of a defender's matchup: how an attack type fares against the defense.
#[derive(Props, Clone, PartialEq)]
pub struct DefenseMatchupProps {
    pub attack_type: AttackType,
    pub defense_type: DefenseType,
}
