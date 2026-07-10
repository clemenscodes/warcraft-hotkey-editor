use warcraft_api::AttackType;

/// The published `View` contract mirroring [`DamageMatchupRowProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct DamageMatchupRowView {
    pub attack_type: AttackType,
}

impl ddd::View for DamageMatchupRowView {}
