use super::view::HeroLevelMenuView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct HeroLevelMenuModel {
    pub level_picker_open: Signal<bool>,
}

impl From<&HeroLevelMenuView> for HeroLevelMenuModel {
    fn from(view: &HeroLevelMenuView) -> Self {
        let HeroLevelMenuView { level_picker_open } = view.clone();
        Self { level_picker_open }
    }
}

impl ddd::Model for HeroLevelMenuModel {
    type View = HeroLevelMenuView;
}
