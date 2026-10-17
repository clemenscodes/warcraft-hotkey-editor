use warcraft_api::{Mana, ManaRegen};

#[derive(Clone, PartialEq)]
pub struct ManaRowView {
    pub value: Mana,
    pub regen: ManaRegen,
}

impl ddd::Layered for ManaRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ManaRowView {}
