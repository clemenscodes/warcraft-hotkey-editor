use super::view::RegularIntelligenceRowView;
use dioxus::prelude::*;
use warcraft_keybinds::{AttributeStatistic, Gain};

/// The regular intelligence row's input: the hero's intelligence at the selected level, its per-level
/// growth, and its label — all shaped in the row's hook so the leaf only places them.
#[derive(Props, Clone, PartialEq)]
pub struct RegularIntelligenceRowProps {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}

impl From<&RegularIntelligenceRowView> for RegularIntelligenceRowProps {
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

impl ddd::Props for RegularIntelligenceRowProps {
    type View = RegularIntelligenceRowView;
}
