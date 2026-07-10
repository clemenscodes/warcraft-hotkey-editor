use super::props::AgilityRowProps;
use warcraft_api::PrimaryAttribute;
use warcraft_keybinds::{AttributeStatistic, Gain};

/// The shaped agility row figures: the attribute, its per-level growth, and its label.
pub(super) struct AgilityRowModel {
    pub(super) statistic: AttributeStatistic,
    pub(super) growth: Gain,
    pub(super) label: String,
}

pub(super) fn use_agility_row(props: &AgilityRowProps) -> AgilityRowModel {
    let statistic = props.statistic;
    let growth = statistic.growth();
    let attribute = PrimaryAttribute::Agility;
    let label = attribute.to_string();
    AgilityRowModel {
        statistic,
        growth,
        label,
    }
}
