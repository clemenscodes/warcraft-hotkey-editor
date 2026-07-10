use dioxus::prelude::*;
use warcraft_api::AttackType;

/// The attacker's damage matchup grid: how its attack fares against each defense.
#[derive(Props, Clone, PartialEq)]
pub struct DamageMatchupRowProps {
    pub attack_type: AttackType,
}
