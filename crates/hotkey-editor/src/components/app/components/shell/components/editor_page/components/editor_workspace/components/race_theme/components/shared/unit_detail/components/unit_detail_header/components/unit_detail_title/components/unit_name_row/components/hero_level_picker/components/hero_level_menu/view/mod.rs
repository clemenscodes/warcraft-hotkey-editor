use dioxus::prelude::*;

/// The published `View` contract mirroring [`HeroLevelMenuModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct HeroLevelMenuView {
    pub level_picker_open: Signal<bool>,
}

impl ddd::View for HeroLevelMenuView {}
