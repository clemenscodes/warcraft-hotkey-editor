use warcraft_api::{AttackType, DefenseType};

#[derive(Clone, PartialEq)]
pub struct DefenseMatchupView {
    pub attack_type: AttackType,
    pub defense_type: DefenseType,
}

impl ddd::Layered for DefenseMatchupView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for DefenseMatchupView {}
