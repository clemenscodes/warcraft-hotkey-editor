use warcraft_api::{AttributeStatistic, Gain};

#[derive(Clone, PartialEq)]
pub struct PrimaryAgilityRowView {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}

impl ddd::Layered for PrimaryAgilityRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for PrimaryAgilityRowView {}
