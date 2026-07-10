use super::props::IntelligenceRowProps;
use warcraft_api::PrimaryAttribute;
use warcraft_keybinds::{AttributeStatistic, Gain};

/// The shaped intelligence row figures: the attribute, its per-level growth, and its label.
pub(super) struct IntelligenceRowModel {
    pub(super) statistic: AttributeStatistic,
    pub(super) growth: Gain,
    pub(super) label: String,
}

pub(super) fn use_intelligence_row(props: &IntelligenceRowProps) -> IntelligenceRowModel {
    let statistic = props.statistic;
    let growth = statistic.growth();
    let attribute = PrimaryAttribute::Intelligence;
    let label = attribute.to_string();
    IntelligenceRowModel {
        statistic,
        growth,
        label,
    }
}
