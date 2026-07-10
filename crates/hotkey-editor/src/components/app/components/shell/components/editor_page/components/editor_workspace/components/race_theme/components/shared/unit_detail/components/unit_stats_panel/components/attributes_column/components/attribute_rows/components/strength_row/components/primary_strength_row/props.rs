use super::super::super::StrengthRowProps;
use dioxus::prelude::*;
use warcraft_api::PrimaryAttribute;
use warcraft_keybinds::{AttributeStatistic, Gain};

/// The primary strength row's input: the hero's strength at the selected level, its per-level
/// growth, and its label — all resolved by `From` so the leaf only places them.
#[derive(Props, Clone, PartialEq)]
pub struct PrimaryStrengthRowProps {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}

impl From<&StrengthRowProps> for PrimaryStrengthRowProps {
    fn from(props: &StrengthRowProps) -> Self {
        let statistic = props.statistic;
        let growth = statistic.growth();
        let attribute = PrimaryAttribute::Strength;
        let label = attribute.to_string();
        Self {
            statistic,
            growth,
            label,
        }
    }
}
