use warcraft_api::ManaRegen;

#[derive(Clone, PartialEq)]
pub struct ManaRegenGainView {
    pub value: ManaRegen,
}

impl ddd::Layered for ManaRegenGainView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ManaRegenGainView {}
