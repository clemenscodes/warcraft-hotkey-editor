use warcraft_keybinds::AttributeStatistic;

/// The published `View` contract mirroring [`AttributeRowsModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct AttributeRowsView {
    pub strength: AttributeStatistic,
    pub strength_is_primary: bool,
    pub agility: AttributeStatistic,
    pub agility_is_primary: bool,
    pub intelligence: AttributeStatistic,
    pub intelligence_is_primary: bool,
}

impl ddd::View for AttributeRowsView {}
