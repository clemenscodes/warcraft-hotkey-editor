use warcraft_api::AttackSpeed;

#[derive(Clone, PartialEq)]
pub struct AttackSpeedRowView {
    pub value: AttackSpeed,
}

impl ddd::Layered for AttackSpeedRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for AttackSpeedRowView {}
