use warcraft_api::{AttributeStatistic, Gain};

#[derive(Clone, PartialEq)]
pub struct RegularAttributeRowView {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}

impl ddd::Layered for RegularAttributeRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for RegularAttributeRowView {}
