use super::components::hero_level_trigger_number::HeroLevelTriggerNumberProps;
use dioxus::prelude::*;

/// The hero-level dropdown trigger: the current level shown, whether the menu is
/// open (drives the accent + caret), and the toggle handler.
#[derive(Props, Clone, PartialEq)]
pub struct HeroLevelTriggerProps {
    #[props(into)]
    pub number: String,
    pub is_open: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&HeroLevelTriggerProps> for HeroLevelTriggerNumberProps {
    fn from(props: &HeroLevelTriggerProps) -> Self {
        let number = props.number.clone();
        Self { number }
    }
}
