use warcraft_api::AttackType;

#[derive(Clone, PartialEq)]
pub struct DamageMatchupRowView {
    pub attack_type: AttackType,
}

impl ddd::Layered for DamageMatchupRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for DamageMatchupRowView {}
