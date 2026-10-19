use warcraft_api::AttributeStatistic;

#[derive(Clone, PartialEq)]
pub struct StrengthRowView {
    pub statistic: AttributeStatistic,
    pub is_primary: bool,
}

impl ddd::Layered for StrengthRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for StrengthRowView {}
