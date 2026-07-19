use warcraft_api::ManaRegen;

#[derive(Clone, PartialEq)]
pub struct ManaRegenGainView {
    pub value: ManaRegen,
}

impl ddd::View for ManaRegenGainView {}
