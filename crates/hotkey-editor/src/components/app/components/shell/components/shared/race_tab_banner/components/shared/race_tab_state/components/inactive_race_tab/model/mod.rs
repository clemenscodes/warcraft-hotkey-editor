use super::view::InactiveRaceTabView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InactiveRaceTabModel {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&InactiveRaceTabView> for InactiveRaceTabModel {
    fn from(view: &InactiveRaceTabView) -> Self {
        let InactiveRaceTabView {
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

impl ddd::Model for InactiveRaceTabModel {
    type View = InactiveRaceTabView;
}
