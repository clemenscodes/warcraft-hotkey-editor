use super::view::ToggleButtonView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ToggleButtonModel {
    pub label: &'static str,
    pub active: bool,
    #[props(default)]
    pub title: Option<&'static str>,
    pub onclick: EventHandler<MouseEvent>,
    #[props(default)]
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&ToggleButtonView> for ToggleButtonModel {
    fn from(view: &ToggleButtonView) -> Self {
        let ToggleButtonView {
            label,
            active,
            title,
            onclick,
            onkeydown,
        } = view.clone();
        Self {
            label,
            active,
            title,
            onclick,
            onkeydown,
        }
    }
}

impl ddd::Model for ToggleButtonModel {
    type View = ToggleButtonView;
}
