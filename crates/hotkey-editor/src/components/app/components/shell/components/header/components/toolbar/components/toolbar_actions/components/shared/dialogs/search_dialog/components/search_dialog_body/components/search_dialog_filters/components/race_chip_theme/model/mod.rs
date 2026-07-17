use super::view::RaceChipThemeView;
use dioxus::prelude::*;
use warcraft_api::Race;

#[derive(Props, Clone, PartialEq)]
pub struct RaceChipThemeModel {
    pub race: Race,
    pub label: &'static str,
    pub active: bool,
    pub on_pick: EventHandler<MouseEvent>,
}

impl From<&RaceChipThemeView> for RaceChipThemeModel {
    fn from(view: &RaceChipThemeView) -> Self {
        let RaceChipThemeView {
            race,
            label,
            active,
            on_pick,
        } = view.clone();
        Self {
            race,
            label,
            active,
            on_pick,
        }
    }
}

impl ddd::Model for RaceChipThemeModel {
    type View = RaceChipThemeView;
}
