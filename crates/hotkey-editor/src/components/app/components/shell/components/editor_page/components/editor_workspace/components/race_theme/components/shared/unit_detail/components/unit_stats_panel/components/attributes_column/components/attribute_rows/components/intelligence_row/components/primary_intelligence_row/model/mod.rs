use super::view::PrimaryIntelligenceRowView;
use dioxus::prelude::*;
use warcraft_api::{AttributeStatistic, Gain};

/// The primary intelligence row's input: the hero's intelligence at the selected level, its per-level
/// growth, and its label — all shaped in the row's hook so the leaf only places them.
#[derive(Props, Clone, PartialEq)]
pub struct PrimaryIntelligenceRowModel {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}

impl From<&PrimaryIntelligenceRowView> for PrimaryIntelligenceRowModel {
    fn from(view: &PrimaryIntelligenceRowView) -> Self {
        let PrimaryIntelligenceRowView {
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

impl ddd::Model for PrimaryIntelligenceRowModel {
    type View = PrimaryIntelligenceRowView;
}
