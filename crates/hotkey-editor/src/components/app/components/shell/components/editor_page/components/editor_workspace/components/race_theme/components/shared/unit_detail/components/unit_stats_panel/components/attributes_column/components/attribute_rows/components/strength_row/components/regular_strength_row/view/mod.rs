use warcraft_api::{AttributeStatistic, Gain};

/// The published `View` contract mirroring [`RegularStrengthRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct RegularStrengthRowView {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}

impl ddd::View for RegularStrengthRowView {}
