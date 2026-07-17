use super::view::HumanRaceChipThemeView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HumanRaceChipThemeModel {
    pub label: &'static str,
    pub active: bool,
    pub on_pick: EventHandler<MouseEvent>,
}

impl From<&HumanRaceChipThemeView> for HumanRaceChipThemeModel {
    fn from(view: &HumanRaceChipThemeView) -> Self {
        let HumanRaceChipThemeView {
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

impl ddd::Model for HumanRaceChipThemeModel {
    type View = HumanRaceChipThemeView;
}
