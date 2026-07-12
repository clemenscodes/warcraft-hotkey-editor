use warcraft_api::{AttackType, DefenseType};

/// The published `View` contract mirroring [`DefenseMatchupModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct DefenseMatchupView {
    pub attack_type: AttackType,
    pub defense_type: DefenseType,
}

impl ddd::View for DefenseMatchupView {}
