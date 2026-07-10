use super::view::IdleToggleButtonView;
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

impl From<&IdleToggleButtonView> for IdleToggleButtonProps {
    fn from(view: &IdleToggleButtonView) -> Self {
        let IdleToggleButtonView {
            label,
            title,
            onclick,
            onkeydown,
        } = view.clone();
        Self {
            label,
            title,
            onclick,
            onkeydown,
        }
    }
}

impl ddd::Props for IdleToggleButtonProps {
    type View = IdleToggleButtonView;
}
