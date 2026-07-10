use super::view::PrimaryIntelligenceRowView;
use dioxus::prelude::*;
use warcraft_keybinds::{AttributeStatistic, Gain};

/// The primary intelligence row's input: the hero's intelligence at the selected level, its per-level
/// growth, and its label — all shaped in the row's hook so the leaf only places them.
#[derive(Props, Clone, PartialEq)]
pub struct PrimaryIntelligenceRowProps {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}

impl From<&PrimaryIntelligenceRowView> for PrimaryIntelligenceRowProps {
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

impl ddd::Props for PrimaryIntelligenceRowProps {
    type View = PrimaryIntelligenceRowView;
}
