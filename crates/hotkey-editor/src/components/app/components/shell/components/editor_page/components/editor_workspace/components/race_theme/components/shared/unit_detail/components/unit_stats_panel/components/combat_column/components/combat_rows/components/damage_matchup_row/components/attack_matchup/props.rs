use dioxus::prelude::*;
use warcraft_api::{AttackType, DefenseType};

/// One cell of an attacker's damage matchup: how the attack fares against a defense.
#[derive(Props, Clone, PartialEq)]
pub struct AttackMatchupProps {
    pub defense_type: DefenseType,
    pub attack_type: AttackType,
}
