use super::view::HeroLevelOptionView;
use dioxus::prelude::*;

/// One selectable hero level in the dropdown: which level it offers, and the menu's
/// open signal it closes when chosen. Whether it is the current level and the level it
/// writes are read from editor context, so the selected-level signal is not a prop.
#[derive(Props, Clone, PartialEq)]
pub struct HeroLevelOptionModel {
    pub level_index: u32,
    pub level_picker_open: Signal<bool>,
}

impl From<&HeroLevelOptionView> for HeroLevelOptionModel {
    fn from(view: &HeroLevelOptionView) -> Self {
        let HeroLevelOptionView {
            level_index,
            level_picker_open,
        } = view.clone();
        Self {
            level_index,
            level_picker_open,
        }
    }
}

impl ddd::Model for HeroLevelOptionModel {
    type View = HeroLevelOptionView;
}
