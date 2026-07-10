use super::view::PrimaryStrengthRowView;
use dioxus::prelude::*;
use warcraft_keybinds::{AttributeStatistic, Gain};

/// The primary strength row's input: the hero's strength at the selected level, its per-level
/// growth, and its label — all shaped in the row's hook so the leaf only places them.
#[derive(Props, Clone, PartialEq)]
pub struct PrimaryStrengthRowProps {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}

impl From<&PrimaryStrengthRowView> for PrimaryStrengthRowProps {
    fn from(view: &PrimaryStrengthRowView) -> Self {
        let PrimaryStrengthRowView {
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

impl ddd::Props for PrimaryStrengthRowProps {
    type View = PrimaryStrengthRowView;
}
