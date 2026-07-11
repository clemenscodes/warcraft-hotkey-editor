use super::view::RegularStrengthRowView;
use dioxus::prelude::*;
use warcraft_api::{AttributeStatistic, Gain};

/// The regular strength row's input: the hero's strength at the selected level, its per-level
/// growth, and its label — all shaped in the row's hook so the leaf only places them.
#[derive(Props, Clone, PartialEq)]
pub struct RegularStrengthRowModel {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}

impl From<&RegularStrengthRowView> for RegularStrengthRowModel {
    fn from(view: &RegularStrengthRowView) -> Self {
        let RegularStrengthRowView {
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

impl ddd::Model for RegularStrengthRowModel {
    type View = RegularStrengthRowView;
}
