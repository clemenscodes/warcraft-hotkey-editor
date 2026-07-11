use dioxus::prelude::*;

/// The published `View` contract mirroring [`HeroLevelOptionModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HeroLevelOptionView {
    pub level_index: u32,
    pub level_picker_open: Signal<bool>,
}

impl ddd::View for HeroLevelOptionView {}
