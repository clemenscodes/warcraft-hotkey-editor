use super::view::HeroLevelOptionView;
use dioxus::prelude::*;

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
