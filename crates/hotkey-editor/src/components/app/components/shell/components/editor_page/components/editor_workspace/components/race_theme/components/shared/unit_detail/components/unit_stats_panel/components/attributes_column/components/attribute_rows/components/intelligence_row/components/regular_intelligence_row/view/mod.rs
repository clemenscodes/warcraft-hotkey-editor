use warcraft_keybinds::{AttributeStatistic, Gain};

/// The published `View` contract mirroring [`RegularIntelligenceRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct RegularIntelligenceRowView {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}

impl ddd::View for RegularIntelligenceRowView {}
