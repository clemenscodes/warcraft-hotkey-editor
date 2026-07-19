use warcraft_api::AttackType;

#[derive(Clone, PartialEq)]
pub struct DamageMatchupRowView {
    pub attack_type: AttackType,
}

impl ddd::View for DamageMatchupRowView {}
