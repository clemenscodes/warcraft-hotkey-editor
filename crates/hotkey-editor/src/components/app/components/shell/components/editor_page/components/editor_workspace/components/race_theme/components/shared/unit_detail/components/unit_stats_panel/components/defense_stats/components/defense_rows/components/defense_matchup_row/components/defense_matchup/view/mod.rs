use warcraft_api::{AttackType, DefenseType};

#[derive(Clone, PartialEq)]
pub struct DefenseMatchupView {
    pub attack_type: AttackType,
    pub defense_type: DefenseType,
}

impl ddd::View for DefenseMatchupView {}
