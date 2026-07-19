use warcraft_api::HeroStatistics;

#[derive(Clone, PartialEq)]
pub struct AttributesStatsView {
    pub hero: Option<HeroStatistics>,
}

impl ddd::View for AttributesStatsView {}
