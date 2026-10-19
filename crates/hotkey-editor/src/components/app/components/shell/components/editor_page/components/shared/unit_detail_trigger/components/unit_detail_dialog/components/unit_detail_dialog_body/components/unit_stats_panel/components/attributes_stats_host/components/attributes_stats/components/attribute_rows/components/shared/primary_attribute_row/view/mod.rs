use warcraft_api::{AttributeStatistic, Gain};

#[derive(Clone, PartialEq)]
pub struct PrimaryAttributeRowView {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}

impl ddd::Layered for PrimaryAttributeRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PrimaryAttributeRowView {}
