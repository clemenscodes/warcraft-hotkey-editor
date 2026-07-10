use super::props::StrengthRowProps;
use warcraft_api::PrimaryAttribute;
use warcraft_keybinds::{AttributeStatistic, Gain};

/// The shaped strength row figures: the attribute, its per-level growth, and its label.
pub(super) struct StrengthRowModel {
    pub(super) statistic: AttributeStatistic,
    pub(super) growth: Gain,
    pub(super) label: String,
}

pub(super) fn use_strength_row(props: &StrengthRowProps) -> StrengthRowModel {
    let statistic = props.statistic;
    let growth = statistic.growth();
    let attribute = PrimaryAttribute::Strength;
    let label = attribute.to_string();
    StrengthRowModel {
        statistic,
        growth,
        label,
    }
}
