use dioxus::prelude::*;

/// The idle toggle button's props: the label, optional tooltip, and activation
/// handlers. It is the pill's resting look; the active look is its sibling component.
#[derive(Props, Clone, PartialEq)]
pub struct IdleToggleButtonProps {
    pub label: &'static str,
    pub title: Option<&'static str>,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}
