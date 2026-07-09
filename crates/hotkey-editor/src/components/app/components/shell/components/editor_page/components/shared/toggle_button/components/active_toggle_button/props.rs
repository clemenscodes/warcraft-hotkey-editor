use dioxus::prelude::*;

/// The active toggle button's props: the label, optional tooltip, and activation
/// handlers. It is the pill lit gold to show it is the current choice in its group.
#[derive(Props, Clone, PartialEq)]
pub struct ActiveToggleButtonProps {
    pub label: &'static str,
    pub title: Option<&'static str>,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}
