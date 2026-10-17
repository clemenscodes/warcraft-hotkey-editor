use warcraft_api::Mana;

#[derive(Clone, PartialEq)]
pub struct ManaValueView {
    pub value: Mana,
}

impl ddd::Layered for ManaValueView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ManaValueView {}
