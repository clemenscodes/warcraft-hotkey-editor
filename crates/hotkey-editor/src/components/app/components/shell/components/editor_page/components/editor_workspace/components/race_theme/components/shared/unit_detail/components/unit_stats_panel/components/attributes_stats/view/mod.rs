use warcraft_api::HeroStatistics;

/// The published `View` contract mirroring [`AttributesStatsModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct AttributesStatsView {
    pub hero: Option<HeroStatistics>,
}

impl ddd::View for AttributesStatsView {}
