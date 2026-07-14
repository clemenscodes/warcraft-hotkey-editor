use super::view::ModeTabView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ModeTabModel {
    pub label: &'static str,
    pub active: bool,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&ModeTabView> for ModeTabModel {
    fn from(view: &ModeTabView) -> Self {
        let ModeTabView {
            label,
            active,
            onclick,
            onkeydown,
        } = view.clone();
        Self {
            label,
            active,
            onclick,
            onkeydown,
        }
    }
}

impl ddd::Model for ModeTabModel {
    type View = ModeTabView;
}
