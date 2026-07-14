use super::view::IdleToggleButtonView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct IdleToggleButtonModel {
    pub label: &'static str,
    pub title: Option<&'static str>,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&IdleToggleButtonView> for IdleToggleButtonModel {
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

impl ddd::Model for IdleToggleButtonModel {
    type View = IdleToggleButtonView;
}
