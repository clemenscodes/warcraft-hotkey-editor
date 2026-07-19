use warcraft_api::{AttributeStatistic, Gain};

#[derive(Clone, PartialEq)]
pub struct PrimaryIntelligenceRowView {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}

impl ddd::View for PrimaryIntelligenceRowView {}
