use warcraft_keybinds::AttributeStatistic;

/// The published `View` contract mirroring [`IntelligenceRowProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct IntelligenceRowView {
    pub statistic: AttributeStatistic,
    pub is_primary: bool,
}

impl ddd::View for IntelligenceRowView {}
