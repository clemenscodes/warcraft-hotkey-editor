use super::view::ActiveRaceTabView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ActiveRaceTabModel {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&ActiveRaceTabView> for ActiveRaceTabModel {
    fn from(view: &ActiveRaceTabView) -> Self {
        let ActiveRaceTabView {
            label,
            onclick,
            onkeydown,
        } = view.clone();
        Self {
            label,
            onclick,
            onkeydown,
        }
    }
}

impl ddd::Model for ActiveRaceTabModel {
    type View = ActiveRaceTabView;
}
