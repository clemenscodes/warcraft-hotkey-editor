use super::view::HeroLevelTriggerNumberView;
use dioxus::prelude::*;

/// The current hero level shown in the centre of the trigger.
#[derive(Props, Clone, PartialEq)]
pub struct HeroLevelTriggerNumberProps {
    #[props(into)]
    pub number: String,
}

impl From<&HeroLevelTriggerNumberView> for HeroLevelTriggerNumberProps {
    fn from(view: &HeroLevelTriggerNumberView) -> Self {
        let HeroLevelTriggerNumberView { number } = view.clone();
        Self { number }
    }
}

impl ddd::Props for HeroLevelTriggerNumberProps {
    type View = HeroLevelTriggerNumberView;
}
