use super::view::HeroLevelTriggerView;
use dioxus::prelude::*;

/// The hero-level dropdown trigger: the current level shown, whether the menu is
/// open (selects the open xor resting look), and the toggle handler.
#[derive(Props, Clone, PartialEq)]
pub struct HeroLevelTriggerModel {
    #[props(into)]
    pub number: String,
    pub is_open: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&HeroLevelTriggerView> for HeroLevelTriggerModel {
    fn from(view: &HeroLevelTriggerView) -> Self {
        let HeroLevelTriggerView {
            number,
            is_open,
            onclick,
        } = view.clone();
        Self {
            number,
            is_open,
            onclick,
        }
    }
}

impl ddd::Model for HeroLevelTriggerModel {
    type View = HeroLevelTriggerView;
}
