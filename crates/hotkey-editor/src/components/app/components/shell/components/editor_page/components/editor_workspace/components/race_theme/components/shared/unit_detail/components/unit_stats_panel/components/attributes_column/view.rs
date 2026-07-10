use warcraft_keybinds::HeroStatistics;

/// The published `View` contract mirroring [`AttributesColumnProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct AttributesColumnView {
    pub hero: Option<HeroStatistics>,
}

impl ddd::View for AttributesColumnView {}
