use warcraft_api::Mana;

#[derive(Clone, PartialEq)]
pub struct ManaRowView {
    pub value: Mana,
}

impl ddd::Layered for ManaRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ManaRowView {}
