use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct HeroLevelMenuView {
    pub level_picker_open: Signal<bool>,
}

impl ddd::View for HeroLevelMenuView {}
