use super::super::super::AgilityRowProps;
use dioxus::prelude::*;
use warcraft_api::PrimaryAttribute;
use warcraft_keybinds::{AttributeStatistic, Gain};

/// The regular agility row's input: the hero's agility at the selected level, its per-level
/// growth, and its label — all resolved by `From` so the leaf only places them.
#[derive(Props, Clone, PartialEq)]
pub struct RegularAgilityRowProps {
    pub statistic: AttributeStatistic,
    pub growth: Gain,
    pub label: String,
}

impl From<&AgilityRowProps> for RegularAgilityRowProps {
    fn from(props: &AgilityRowProps) -> Self {
        let statistic = props.statistic;
        let growth = statistic.growth();
        let attribute = PrimaryAttribute::Agility;
        let label = attribute.to_string();
        Self {
            statistic,
            growth,
            label,
        }
    }
}
