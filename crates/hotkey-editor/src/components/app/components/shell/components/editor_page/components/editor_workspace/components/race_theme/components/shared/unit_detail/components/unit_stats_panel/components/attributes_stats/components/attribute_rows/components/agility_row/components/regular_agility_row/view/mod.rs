use warcraft_api::{AttributeStatistic, Gain};

/// The published `View` contract mirroring [`RegularAgilityRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct RegularAgilityRowView {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}

impl ddd::View for RegularAgilityRowView {}
