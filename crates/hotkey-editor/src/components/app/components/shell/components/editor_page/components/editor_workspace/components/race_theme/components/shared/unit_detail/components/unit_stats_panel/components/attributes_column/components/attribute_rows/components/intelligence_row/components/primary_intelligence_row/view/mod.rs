use warcraft_keybinds::{AttributeStatistic, Gain};

/// The published `View` contract mirroring [`PrimaryIntelligenceRowModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct PrimaryIntelligenceRowView {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}

impl ddd::View for PrimaryIntelligenceRowView {}
