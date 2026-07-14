use super::view::RaceTabsView;
use dioxus::prelude::*;
use warcraft_api::Race;

#[derive(Props, Clone, Copy, PartialEq)]
pub struct RaceTabsModel {
    pub active_race: Signal<Race>,
    pub on_select: EventHandler<Race>,
}

#[derive(Props, Clone, PartialEq)]
pub struct RaceTabBinding {
    pub is_active: bool,
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl From<&RaceTabsView> for RaceTabsModel {
    fn from(view: &RaceTabsView) -> Self {
        let RaceTabsView {
            active_race,
            on_select,
        } = view.clone();
        Self {
            active_race,
            on_select,
        }
    }
}

impl ddd::Model for RaceTabsModel {
    type View = RaceTabsView;
}
