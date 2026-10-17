use warcraft_api::{AttributeStatistic, Gain};

#[derive(Clone, PartialEq)]
pub struct RegularIntelligenceRowView {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}

impl ddd::Layered for RegularIntelligenceRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for RegularIntelligenceRowView {}
