use warcraft_api::HeroStatistics;

#[derive(Clone, PartialEq)]
pub struct AttributesStatsView {
    pub hero: Option<HeroStatistics>,
}

impl ddd::Layered for AttributesStatsView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for AttributesStatsView {}
