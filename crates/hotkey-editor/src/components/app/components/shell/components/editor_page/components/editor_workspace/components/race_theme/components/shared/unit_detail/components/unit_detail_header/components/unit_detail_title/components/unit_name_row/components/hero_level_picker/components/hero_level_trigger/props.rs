use super::components::closed_hero_level_trigger::ClosedHeroLevelTriggerProps;
use super::components::open_hero_level_trigger::OpenHeroLevelTriggerProps;
use dioxus::prelude::*;

/// The hero-level dropdown trigger: the current level shown, whether the menu is
/// open (selects the open xor resting look), and the toggle handler.
#[derive(Props, Clone, PartialEq)]
pub struct HeroLevelTriggerProps {
    #[props(into)]
    pub number: String,
    pub is_open: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&HeroLevelTriggerProps> for OpenHeroLevelTriggerProps {
    fn from(props: &HeroLevelTriggerProps) -> Self {
        let number = props.number.clone();
        let onclick = props.onclick;
        Self { number, onclick }
    }
}

impl From<&HeroLevelTriggerProps> for ClosedHeroLevelTriggerProps {
    fn from(props: &HeroLevelTriggerProps) -> Self {
        let number = props.number.clone();
        let onclick = props.onclick;
        Self { number, onclick }
    }
}
