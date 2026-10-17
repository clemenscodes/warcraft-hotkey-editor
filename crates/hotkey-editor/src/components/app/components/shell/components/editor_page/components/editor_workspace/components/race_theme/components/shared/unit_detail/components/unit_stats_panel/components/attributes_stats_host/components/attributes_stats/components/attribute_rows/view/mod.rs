use warcraft_api::AttributeStatistic;

#[derive(Clone, PartialEq)]
pub struct AttributeRowsView {
    pub strength: AttributeStatistic,
    pub strength_is_primary: bool,
    pub agility: AttributeStatistic,
    pub agility_is_primary: bool,
    pub intelligence: AttributeStatistic,
    pub intelligence_is_primary: bool,
}

impl ddd::Layered for AttributeRowsView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for AttributeRowsView {}
