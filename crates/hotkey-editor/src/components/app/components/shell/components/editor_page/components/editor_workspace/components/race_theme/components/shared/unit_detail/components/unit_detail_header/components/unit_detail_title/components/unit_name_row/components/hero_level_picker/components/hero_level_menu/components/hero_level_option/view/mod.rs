use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct HeroLevelOptionView {
    pub level_index: u32,
    pub level_picker_open: Signal<bool>,
}

impl ddd::View for HeroLevelOptionView {}
