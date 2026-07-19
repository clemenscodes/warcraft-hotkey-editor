use warcraft_api::ManaRegen;

#[derive(Clone, PartialEq)]
pub struct ManaRegenRowView {
    pub value: ManaRegen,
}

impl ddd::View for ManaRegenRowView {}
