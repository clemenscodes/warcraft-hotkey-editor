use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct HeroLevelMenuView {
    pub on_close: EventHandler<()>,
}

impl ddd::Layered for HeroLevelMenuView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HeroLevelMenuView {}
