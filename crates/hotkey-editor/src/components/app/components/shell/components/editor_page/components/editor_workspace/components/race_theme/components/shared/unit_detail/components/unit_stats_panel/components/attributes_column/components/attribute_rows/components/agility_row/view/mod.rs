use warcraft_keybinds::AttributeStatistic;

/// The published `View` contract mirroring [`AgilityRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct AgilityRowView {
    pub statistic: AttributeStatistic,
    pub is_primary: bool,
}

impl ddd::View for AgilityRowView {}
