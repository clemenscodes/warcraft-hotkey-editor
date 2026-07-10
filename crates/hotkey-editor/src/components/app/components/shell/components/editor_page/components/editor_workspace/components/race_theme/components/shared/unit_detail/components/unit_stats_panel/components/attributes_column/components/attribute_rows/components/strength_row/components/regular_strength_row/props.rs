use dioxus::prelude::*;
use warcraft_keybinds::{AttributeStatistic, Gain};

/// The regular strength row's input: the hero's strength at the selected level, its per-level
/// growth, and its label — all shaped in the row's hook so the leaf only places them.
#[derive(Props, Clone, PartialEq)]
pub struct RegularStrengthRowProps {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}
