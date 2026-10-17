use warcraft_api::DamageRange;

#[derive(Clone, PartialEq)]
pub struct DamageRowView {
    pub value: DamageRange,
}

impl ddd::Layered for DamageRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for DamageRowView {}
