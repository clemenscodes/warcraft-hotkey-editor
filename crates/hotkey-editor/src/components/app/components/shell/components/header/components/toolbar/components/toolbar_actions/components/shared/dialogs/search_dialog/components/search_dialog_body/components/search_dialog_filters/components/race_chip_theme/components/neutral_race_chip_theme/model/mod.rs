use super::view::NeutralRaceChipThemeView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct NeutralRaceChipThemeModel {
    pub label: &'static str,
    pub active: bool,
    pub on_pick: EventHandler<MouseEvent>,
}

impl From<&NeutralRaceChipThemeView> for NeutralRaceChipThemeModel {
    fn from(view: &NeutralRaceChipThemeView) -> Self {
        let NeutralRaceChipThemeView {
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

impl ddd::Model for NeutralRaceChipThemeModel {
    type View = NeutralRaceChipThemeView;
}
