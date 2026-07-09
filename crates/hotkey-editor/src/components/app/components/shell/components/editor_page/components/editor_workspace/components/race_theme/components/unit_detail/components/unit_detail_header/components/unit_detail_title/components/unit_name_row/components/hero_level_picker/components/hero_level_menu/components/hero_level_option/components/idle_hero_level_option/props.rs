use dioxus::prelude::*;

/// The idle hero-level option's props: its label and select handler.
#[derive(Props, Clone, PartialEq)]
pub struct IdleHeroLevelOptionProps {
    #[props(into)]
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
}
