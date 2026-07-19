use crate::components::app::components::shell::components::shared::race_tab_banner::components::shared::race_tab_input::RaceTabInputView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct NeutralRaceTabModel {
    pub is_active: bool,
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&RaceTabInputView> for NeutralRaceTabModel {
    fn from(view: &RaceTabInputView) -> Self {
        let RaceTabInputView {
            is_active,
            label,
            onclick,
            onkeydown,
        } = view.clone();
        Self {
            is_active,
            label,
            onclick,
            onkeydown,
        }
    }
}

impl ddd::Model for NeutralRaceTabModel {
    type View = RaceTabInputView;
}
