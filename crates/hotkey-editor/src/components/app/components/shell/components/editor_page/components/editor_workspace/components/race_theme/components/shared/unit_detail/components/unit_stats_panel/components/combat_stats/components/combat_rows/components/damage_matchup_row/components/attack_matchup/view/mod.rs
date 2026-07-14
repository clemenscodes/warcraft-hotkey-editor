use warcraft_api::{AttackType, DefenseType};

#[derive(Clone, PartialEq)]
pub struct AttackMatchupView {
    pub defense_type: DefenseType,
    pub attack_type: AttackType,
}

impl ddd::View for AttackMatchupView {}
