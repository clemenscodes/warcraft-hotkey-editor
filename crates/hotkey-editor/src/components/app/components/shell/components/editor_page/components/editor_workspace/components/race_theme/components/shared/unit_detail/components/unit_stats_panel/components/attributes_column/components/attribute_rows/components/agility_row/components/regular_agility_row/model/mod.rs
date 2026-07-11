use super::view::RegularAgilityRowView;
use dioxus::prelude::*;
use warcraft_keybinds::{AttributeStatistic, Gain};

/// The regular agility row's input: the hero's agility at the selected level, its per-level
/// growth, and its label — all shaped in the row's hook so the leaf only places them.
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
