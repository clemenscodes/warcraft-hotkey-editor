use super::model::AgilityRowModel;
use warcraft_api::PrimaryAttribute;
use warcraft_api::{AttributeStatistic, Gain};

pub(super) struct AgilityRowPresentation {
    pub(super) statistic: AttributeStatistic,
    pub(super) growth: Gain,
    pub(super) label: String,
}

pub(super) fn use_agility_row(props: &AgilityRowModel) -> AgilityRowPresentation {
    let statistic = props.statistic;
    let growth = statistic.growth();
    let attribute = PrimaryAttribute::Agility;
    let label = attribute.to_string();
    AgilityRowPresentation {
        statistic,
        growth,
        label,
    }
}

impl ddd::Presentation for AgilityRowPresentation {
    type Model = AgilityRowModel;
}
