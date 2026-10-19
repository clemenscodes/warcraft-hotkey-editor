use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BurgerCloseView {
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for BurgerCloseView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BurgerCloseView {}
