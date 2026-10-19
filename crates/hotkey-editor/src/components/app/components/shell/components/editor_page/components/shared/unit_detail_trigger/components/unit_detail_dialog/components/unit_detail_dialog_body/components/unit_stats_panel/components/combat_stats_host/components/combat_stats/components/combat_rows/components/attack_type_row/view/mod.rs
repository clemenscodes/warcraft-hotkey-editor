use warcraft_api::AttackType;

#[derive(Clone, PartialEq)]
pub struct AttackTypeRowView {
    pub value: AttackType,
}

impl ddd::Layered for AttackTypeRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for AttackTypeRowView {}
