use warcraft_api::AttributeStatistic;

#[derive(Clone, PartialEq)]
pub struct IntelligenceRowView {
    pub statistic: AttributeStatistic,
    pub is_primary: bool,
}

impl ddd::View for IntelligenceRowView {}
