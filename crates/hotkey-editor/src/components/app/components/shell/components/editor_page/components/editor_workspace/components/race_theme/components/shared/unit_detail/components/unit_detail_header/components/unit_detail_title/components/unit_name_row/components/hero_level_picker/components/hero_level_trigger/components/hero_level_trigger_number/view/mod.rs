#[derive(Clone, PartialEq)]
pub struct HeroLevelTriggerNumberView {
    pub number: String,
}

impl ddd::View for HeroLevelTriggerNumberView {}
