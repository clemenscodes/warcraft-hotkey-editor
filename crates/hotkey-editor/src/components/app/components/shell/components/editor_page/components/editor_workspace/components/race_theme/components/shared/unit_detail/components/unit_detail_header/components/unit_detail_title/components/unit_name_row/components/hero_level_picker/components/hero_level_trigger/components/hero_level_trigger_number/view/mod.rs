/// The published `View` contract mirroring [`HeroLevelTriggerNumberModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HeroLevelTriggerNumberView {
    pub number: String,
}

impl ddd::View for HeroLevelTriggerNumberView {}
