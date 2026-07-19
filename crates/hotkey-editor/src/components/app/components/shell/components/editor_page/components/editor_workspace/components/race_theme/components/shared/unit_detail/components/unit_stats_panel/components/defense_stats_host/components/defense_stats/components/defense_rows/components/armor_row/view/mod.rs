use warcraft_api::ArmorFigure as Armor;

#[derive(Clone, PartialEq)]
pub struct ArmorRowView {
    pub value: Armor,
}

impl ddd::View for ArmorRowView {}
