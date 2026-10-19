use warcraft_api::AttributeStatistic;

#[derive(Clone, PartialEq)]
pub struct IntelligenceRowView {
    pub statistic: AttributeStatistic,
    pub is_primary: bool,
}

impl ddd::Layered for IntelligenceRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for IntelligenceRowView {}
