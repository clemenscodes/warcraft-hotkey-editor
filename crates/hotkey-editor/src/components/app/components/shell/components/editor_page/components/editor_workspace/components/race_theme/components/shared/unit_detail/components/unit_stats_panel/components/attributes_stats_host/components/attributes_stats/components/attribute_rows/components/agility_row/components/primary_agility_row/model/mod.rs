use super::view::PrimaryAgilityRowView;
use dioxus::prelude::*;
use warcraft_api::{AttributeStatistic, Gain};

#[derive(Props, Clone, PartialEq)]
pub struct PrimaryAgilityRowModel {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}

impl From<&PrimaryAgilityRowView> for PrimaryAgilityRowModel {
    fn from(view: &PrimaryAgilityRowView) -> Self {
        let PrimaryAgilityRowView {
            statistic,
            growth,
            label,
        } = view.clone();
        Self {
            statistic,
            growth,
            label,
        }
    }
}

impl ddd::Model for PrimaryAgilityRowModel {
    type View = PrimaryAgilityRowView;
}
