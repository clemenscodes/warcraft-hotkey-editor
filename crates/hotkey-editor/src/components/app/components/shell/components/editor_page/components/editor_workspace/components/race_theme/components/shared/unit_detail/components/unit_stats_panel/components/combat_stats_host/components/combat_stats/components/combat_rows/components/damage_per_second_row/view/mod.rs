use warcraft_api::DamagePerSecond;

#[derive(Clone, PartialEq)]
pub struct DamagePerSecondRowView {
    pub value: Option<DamagePerSecond>,
}

impl ddd::Layered for DamagePerSecondRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for DamagePerSecondRowView {}
