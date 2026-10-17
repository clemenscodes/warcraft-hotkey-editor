use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BurgerDrawerBodyView {
    pub on_close: EventHandler<MouseEvent>,
}

impl ddd::Layered for BurgerDrawerBodyView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BurgerDrawerBodyView {}
