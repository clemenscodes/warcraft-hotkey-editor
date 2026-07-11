use warcraft_api::{AttackType, DefenseType};

/// The published `View` contract mirroring [`AttackMatchupModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct AttackMatchupView {
    pub defense_type: DefenseType,
    pub attack_type: AttackType,
}

impl ddd::View for AttackMatchupView {}
