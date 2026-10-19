use warcraft_api::ArmorFigure as Armor;

#[derive(Clone, PartialEq)]
pub struct ArmorRowView {
    pub value: Armor,
}

impl ddd::Layered for ArmorRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ArmorRowView {}
