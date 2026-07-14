use super::view::ActiveToggleButtonView;
use dioxus::prelude::*;

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
