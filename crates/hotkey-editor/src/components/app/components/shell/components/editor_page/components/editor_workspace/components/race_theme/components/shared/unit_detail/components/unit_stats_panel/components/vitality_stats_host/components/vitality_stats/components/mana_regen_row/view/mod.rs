use warcraft_api::ManaRegen;

#[derive(Clone, PartialEq)]
pub struct ManaRegenRowView {
    pub value: ManaRegen,
}

impl ddd::Layered for ManaRegenRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ManaRegenRowView {}
