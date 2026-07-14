use super::view::HeroLevelTriggerNumberView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HeroLevelTriggerNumberModel {
    #[props(into)]
    pub number: String,
}

impl From<&HeroLevelTriggerNumberView> for HeroLevelTriggerNumberModel {
    fn from(view: &HeroLevelTriggerNumberView) -> Self {
        let HeroLevelTriggerNumberView { number } = view.clone();
        Self { number }
    }
}

impl ddd::Model for HeroLevelTriggerNumberModel {
    type View = HeroLevelTriggerNumberView;
}
