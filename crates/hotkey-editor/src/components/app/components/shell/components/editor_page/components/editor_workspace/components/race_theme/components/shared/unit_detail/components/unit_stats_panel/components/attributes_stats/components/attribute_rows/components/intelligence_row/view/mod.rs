use warcraft_api::AttributeStatistic;

/// The published `View` contract mirroring [`IntelligenceRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct IntelligenceRowView {
    pub statistic: AttributeStatistic,
    pub is_primary: bool,
}

impl ddd::View for IntelligenceRowView {}
