use super::view::UndeadRaceChipThemeView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UndeadRaceChipThemeModel {
    pub label: &'static str,
    pub active: bool,
    pub on_pick: EventHandler<MouseEvent>,
}

impl From<&UndeadRaceChipThemeView> for UndeadRaceChipThemeModel {
    fn from(view: &UndeadRaceChipThemeView) -> Self {
        let UndeadRaceChipThemeView {
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

impl ddd::Model for UndeadRaceChipThemeModel {
    type View = UndeadRaceChipThemeView;
}
