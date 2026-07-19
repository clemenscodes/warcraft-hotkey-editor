use super::view::RegularIntelligenceRowView;
use dioxus::prelude::*;
use warcraft_api::{AttributeStatistic, Gain};

#[derive(Props, Clone, PartialEq)]
pub struct RegularIntelligenceRowModel {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}

impl From<&RegularIntelligenceRowView> for RegularIntelligenceRowModel {
    fn from(view: &RegularIntelligenceRowView) -> Self {
        let RegularIntelligenceRowView {
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

impl ddd::Model for RegularIntelligenceRowModel {
    type View = RegularIntelligenceRowView;
}
