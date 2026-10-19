use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct HeroLevelBackdropView {
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for HeroLevelBackdropView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for HeroLevelBackdropView {}
