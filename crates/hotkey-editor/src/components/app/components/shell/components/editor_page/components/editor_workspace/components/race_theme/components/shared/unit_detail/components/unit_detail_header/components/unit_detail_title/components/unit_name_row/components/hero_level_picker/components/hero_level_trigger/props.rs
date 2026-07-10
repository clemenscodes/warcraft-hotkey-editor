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
