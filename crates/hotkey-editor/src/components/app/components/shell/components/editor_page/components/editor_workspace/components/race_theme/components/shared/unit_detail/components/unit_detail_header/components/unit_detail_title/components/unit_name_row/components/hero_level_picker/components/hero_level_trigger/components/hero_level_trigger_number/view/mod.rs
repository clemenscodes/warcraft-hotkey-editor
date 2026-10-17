#[derive(Clone, PartialEq)]
pub struct HeroLevelTriggerNumberView {
    pub number: String,
}

impl ddd::Layered for HeroLevelTriggerNumberView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HeroLevelTriggerNumberView {}
