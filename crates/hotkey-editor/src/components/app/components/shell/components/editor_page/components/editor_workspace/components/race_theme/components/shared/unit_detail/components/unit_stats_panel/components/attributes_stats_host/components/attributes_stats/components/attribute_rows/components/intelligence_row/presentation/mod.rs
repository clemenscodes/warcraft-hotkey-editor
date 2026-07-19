use super::model::IntelligenceRowModel;
use warcraft_api::PrimaryAttribute;
use warcraft_api::{AttributeStatistic, Gain};

pub(super) struct IntelligenceRowPresentation {
    pub(super) statistic: AttributeStatistic,
    pub(super) growth: Gain,
    pub(super) label: String,
}

pub(super) fn use_intelligence_row(props: &IntelligenceRowModel) -> IntelligenceRowPresentation {
    let statistic = props.statistic;
    let growth = statistic.growth();
    let attribute = PrimaryAttribute::Intelligence;
    let label = attribute.to_string();
    IntelligenceRowPresentation {
        statistic,
        growth,
        label,
    }
}

impl ddd::Presentation for IntelligenceRowPresentation {
    type Model = IntelligenceRowModel;
}
