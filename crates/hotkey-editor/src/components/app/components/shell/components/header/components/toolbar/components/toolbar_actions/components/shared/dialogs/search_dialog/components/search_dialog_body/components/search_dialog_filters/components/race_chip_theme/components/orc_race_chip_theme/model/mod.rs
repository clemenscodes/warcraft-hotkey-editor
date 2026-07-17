use super::view::OrcRaceChipThemeView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct OrcRaceChipThemeModel {
    pub label: &'static str,
    pub active: bool,
    pub on_pick: EventHandler<MouseEvent>,
}

impl From<&OrcRaceChipThemeView> for OrcRaceChipThemeModel {
    fn from(view: &OrcRaceChipThemeView) -> Self {
        let OrcRaceChipThemeView {
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

impl ddd::Model for OrcRaceChipThemeModel {
    type View = OrcRaceChipThemeView;
}
