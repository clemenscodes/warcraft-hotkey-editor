use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct HeroLevelOptionView {
    pub level_index: u32,
    pub on_close: EventHandler<()>,
}

impl ddd::Layered for HeroLevelOptionView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HeroLevelOptionView {}
