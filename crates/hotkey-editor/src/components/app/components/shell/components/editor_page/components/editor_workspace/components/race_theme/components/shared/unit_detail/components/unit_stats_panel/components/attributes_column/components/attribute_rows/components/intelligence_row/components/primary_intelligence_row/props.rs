use super::super::super::IntelligenceRowProps;
use dioxus::prelude::*;
use warcraft_api::PrimaryAttribute;
use warcraft_keybinds::{AttributeStatistic, Gain};

/// The primary intelligence row's input: the hero's intelligence at the selected level, its per-level
/// growth, and its label — all resolved by `From` so the leaf only places them.
#[derive(Props, Clone, PartialEq)]
pub struct PrimaryIntelligenceRowProps {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}

impl From<&IntelligenceRowProps> for PrimaryIntelligenceRowProps {
    fn from(props: &IntelligenceRowProps) -> Self {
        let statistic = props.statistic;
        let growth = statistic.growth();
        let attribute = PrimaryAttribute::Intelligence;
        let label = attribute.to_string();
        Self {
            statistic,
            growth,
            label,
        }
    }
}
