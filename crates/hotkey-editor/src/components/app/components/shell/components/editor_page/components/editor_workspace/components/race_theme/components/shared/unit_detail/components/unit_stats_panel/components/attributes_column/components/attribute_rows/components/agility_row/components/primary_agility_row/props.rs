use dioxus::prelude::*;
use warcraft_keybinds::{AttributeStatistic, Gain};

/// The primary agility row's input: the hero's agility at the selected level, its per-level
/// growth, and its label — all shaped in the row's hook so the leaf only places them.
#[derive(Props, Clone, PartialEq)]
pub struct PrimaryAgilityRowProps {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}
