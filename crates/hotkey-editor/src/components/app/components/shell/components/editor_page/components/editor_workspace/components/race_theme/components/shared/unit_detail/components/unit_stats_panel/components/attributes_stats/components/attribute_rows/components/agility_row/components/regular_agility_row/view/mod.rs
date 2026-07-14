use warcraft_api::{AttributeStatistic, Gain};

#[derive(Clone, PartialEq)]
pub struct RegularAgilityRowView {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}

impl ddd::View for RegularAgilityRowView {}
