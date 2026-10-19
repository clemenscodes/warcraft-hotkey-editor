use warcraft_api::HeroStatistics;

#[derive(Clone, PartialEq)]
pub struct AttributesStatsHostView {
    pub hero: Option<HeroStatistics>,
}

impl ddd::Layered for AttributesStatsHostView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for AttributesStatsHostView {}
