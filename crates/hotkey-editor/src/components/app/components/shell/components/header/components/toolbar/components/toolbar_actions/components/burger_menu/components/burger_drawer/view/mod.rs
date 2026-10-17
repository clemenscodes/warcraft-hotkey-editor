use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BurgerDrawerView {
    pub on_close: EventHandler<MouseEvent>,
}

impl ddd::Layered for BurgerDrawerView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BurgerDrawerView {}
