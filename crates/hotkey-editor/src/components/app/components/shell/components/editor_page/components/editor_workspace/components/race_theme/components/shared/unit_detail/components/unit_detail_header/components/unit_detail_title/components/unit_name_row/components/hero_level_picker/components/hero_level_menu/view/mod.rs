use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct HeroLevelMenuView {
    pub level_picker_open: Signal<bool>,
}

impl ddd::Layered for HeroLevelMenuView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HeroLevelMenuView {}
