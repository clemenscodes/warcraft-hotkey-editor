use warcraft_api::AttributeStatistic;

#[derive(Clone, PartialEq)]
pub struct AgilityRowView {
    pub statistic: AttributeStatistic,
    pub is_primary: bool,
}

impl ddd::View for AgilityRowView {}
