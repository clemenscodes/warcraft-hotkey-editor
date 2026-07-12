use super::model::StrengthRowModel;
use warcraft_api::PrimaryAttribute;
use warcraft_api::{AttributeStatistic, Gain};

/// The shaped strength row figures: the attribute, its per-level growth, and its label.
pub(super) struct StrengthRowPresentation {
    pub(super) statistic: AttributeStatistic,
    pub(super) growth: Gain,
    pub(super) label: String,
}

pub(super) fn use_strength_row(props: &StrengthRowModel) -> StrengthRowPresentation {
    let statistic = props.statistic;
    let growth = statistic.growth();
    let attribute = PrimaryAttribute::Strength;
    let label = attribute.to_string();
    StrengthRowPresentation {
        statistic,
        growth,
        label,
    }
}

impl ddd::Presentation for StrengthRowPresentation {
    type Model = StrengthRowModel;
}
