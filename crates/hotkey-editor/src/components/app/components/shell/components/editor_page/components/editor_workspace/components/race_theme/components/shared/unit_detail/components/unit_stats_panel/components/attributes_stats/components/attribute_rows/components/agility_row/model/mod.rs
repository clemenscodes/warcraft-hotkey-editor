use super::view::AgilityRowView;
use dioxus::prelude::*;
use warcraft_api::AttributeStatistic;

#[derive(Props, Clone, PartialEq)]
pub struct AgilityRowModel {
    pub statistic: AttributeStatistic,
    pub is_primary: bool,
}

impl From<&AgilityRowView> for AgilityRowModel {
    fn from(view: &AgilityRowView) -> Self {
        let AgilityRowView {
            statistic,
            is_primary,
        } = view.clone();
        Self {
            statistic,
            is_primary,
        }
    }
}

impl ddd::Model for AgilityRowModel {
    type View = AgilityRowView;
}
