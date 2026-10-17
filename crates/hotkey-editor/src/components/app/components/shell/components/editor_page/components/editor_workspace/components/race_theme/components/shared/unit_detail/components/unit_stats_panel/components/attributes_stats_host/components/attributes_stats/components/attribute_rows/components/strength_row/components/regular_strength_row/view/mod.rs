use warcraft_api::{AttributeStatistic, Gain};

#[derive(Clone, PartialEq)]
pub struct RegularStrengthRowView {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}

impl ddd::Layered for RegularStrengthRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for RegularStrengthRowView {}
