use super::view::ActiveToggleButtonView;
use dioxus::prelude::*;

/// The active toggle button's props: the label, optional tooltip, and activation
/// handlers. It is the pill lit gold to show it is the current choice in its group.
#[derive(Props, Clone, PartialEq)]
pub struct ActiveToggleButtonModel {
    pub label: &'static str,
    pub title: Option<&'static str>,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&ActiveToggleButtonView> for ActiveToggleButtonModel {
    fn from(view: &ActiveToggleButtonView) -> Self {
        let ActiveToggleButtonView {
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

impl ddd::Model for ActiveToggleButtonModel {
    type View = ActiveToggleButtonView;
}
