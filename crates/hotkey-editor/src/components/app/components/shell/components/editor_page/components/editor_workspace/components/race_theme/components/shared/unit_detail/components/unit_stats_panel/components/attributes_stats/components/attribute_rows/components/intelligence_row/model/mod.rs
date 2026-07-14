use super::view::IntelligenceRowView;
use dioxus::prelude::*;
use warcraft_api::AttributeStatistic;

#[derive(Props, Clone, PartialEq)]
pub struct IntelligenceRowModel {
    pub statistic: AttributeStatistic,
    pub is_primary: bool,
}

impl From<&IntelligenceRowView> for IntelligenceRowModel {
    fn from(view: &IntelligenceRowView) -> Self {
        let IntelligenceRowView {
            statistic,
            is_primary,
        } = view.clone();
        Self {
            statistic,
            is_primary,
        }
    }
}

impl ddd::Model for IntelligenceRowModel {
    type View = IntelligenceRowView;
}
