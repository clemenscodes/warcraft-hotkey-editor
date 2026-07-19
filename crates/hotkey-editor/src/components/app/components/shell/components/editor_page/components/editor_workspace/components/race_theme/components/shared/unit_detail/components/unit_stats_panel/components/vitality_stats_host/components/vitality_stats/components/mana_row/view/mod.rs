use warcraft_api::Mana;

#[derive(Clone, PartialEq)]
pub struct ManaRowView {
    pub value: Mana,
}

impl ddd::View for ManaRowView {}
