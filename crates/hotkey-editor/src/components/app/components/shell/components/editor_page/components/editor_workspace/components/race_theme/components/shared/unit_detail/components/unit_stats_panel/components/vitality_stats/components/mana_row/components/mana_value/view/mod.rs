use warcraft_api::Mana;

#[derive(Clone, PartialEq)]
pub struct ManaValueView {
    pub value: Mana,
}

impl ddd::View for ManaValueView {}
