use super::view::RegularAgilityRowView;
use dioxus::prelude::*;
use warcraft_api::{AttributeStatistic, Gain};

#[derive(Props, Clone, PartialEq)]
pub struct RegularAgilityRowModel {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}

impl From<&RegularAgilityRowView> for RegularAgilityRowModel {
    fn from(view: &RegularAgilityRowView) -> Self {
        let RegularAgilityRowView {
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

impl ddd::Model for RegularAgilityRowModel {
    type View = RegularAgilityRowView;
}
