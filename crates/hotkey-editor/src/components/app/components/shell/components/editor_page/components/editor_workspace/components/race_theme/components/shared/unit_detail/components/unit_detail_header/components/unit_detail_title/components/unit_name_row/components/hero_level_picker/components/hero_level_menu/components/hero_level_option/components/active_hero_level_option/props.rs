use dioxus::prelude::*;

/// The active hero-level option's props: its label and select handler.
#[derive(Props, Clone, PartialEq)]
pub struct ActiveHeroLevelOptionProps {
    #[props(into)]
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
}
