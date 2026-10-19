use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BurgerBackdropView {
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for BurgerBackdropView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BurgerBackdropView {}
