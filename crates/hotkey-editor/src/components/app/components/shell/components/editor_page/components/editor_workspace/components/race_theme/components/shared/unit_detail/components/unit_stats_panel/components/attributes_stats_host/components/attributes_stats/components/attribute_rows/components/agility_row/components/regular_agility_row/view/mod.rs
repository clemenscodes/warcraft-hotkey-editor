use warcraft_api::{AttributeStatistic, Gain};

#[derive(Clone, PartialEq)]
pub struct RegularAgilityRowView {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}

impl ddd::Layered for RegularAgilityRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for RegularAgilityRowView {}
