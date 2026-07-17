use super::view::NightelfRaceChipThemeView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct NightelfRaceChipThemeModel {
    pub label: &'static str,
    pub active: bool,
    pub on_pick: EventHandler<MouseEvent>,
}

impl From<&NightelfRaceChipThemeView> for NightelfRaceChipThemeModel {
    fn from(view: &NightelfRaceChipThemeView) -> Self {
        let NightelfRaceChipThemeView {
            label,
            active,
            on_pick,
        } = view.clone();
        Self {
            label,
            active,
            on_pick,
        }
    }
}

impl ddd::Model for NightelfRaceChipThemeModel {
    type View = NightelfRaceChipThemeView;
}
