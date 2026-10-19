use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BurgerDrawerHeaderView {
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for BurgerDrawerHeaderView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BurgerDrawerHeaderView {}
