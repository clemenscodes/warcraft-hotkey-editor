use dioxus::prelude::*;

/// The open-look trigger's inputs: the current level caption and the toggle handler.
#[derive(Props, Clone, PartialEq)]
pub struct OpenHeroLevelTriggerProps {
    #[props(into)]
    pub number: String,
    pub onclick: EventHandler<MouseEvent>,
}
