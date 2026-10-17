use warcraft_api::AttributeStatistic;

#[derive(Clone, PartialEq)]
pub struct AgilityRowView {
    pub statistic: AttributeStatistic,
    pub is_primary: bool,
}

impl ddd::Layered for AgilityRowView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for AgilityRowView {}
