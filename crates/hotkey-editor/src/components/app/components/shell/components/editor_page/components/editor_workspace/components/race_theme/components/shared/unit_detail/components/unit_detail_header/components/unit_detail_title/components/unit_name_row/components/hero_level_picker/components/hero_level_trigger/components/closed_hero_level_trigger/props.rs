use dioxus::prelude::*;

/// The resting-look trigger's inputs: the current level caption and the toggle handler.
#[derive(Props, Clone, PartialEq)]
pub struct ClosedHeroLevelTriggerProps {
    #[props(into)]
    pub number: String,
    pub onclick: EventHandler<MouseEvent>,
}
