use warcraft_api::HeroStatistics;

/// The published `View` contract mirroring [`AttributesColumnModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct AttributesColumnView {
    pub hero: Option<HeroStatistics>,
}

impl ddd::View for AttributesColumnView {}
