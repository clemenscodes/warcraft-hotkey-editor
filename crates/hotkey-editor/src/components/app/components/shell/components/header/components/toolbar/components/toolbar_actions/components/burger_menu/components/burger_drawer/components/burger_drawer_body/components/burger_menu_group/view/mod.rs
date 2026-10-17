use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BurgerMenuGroupView {
    pub on_close: EventHandler<MouseEvent>,
}

impl ddd::Layered for BurgerMenuGroupView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BurgerMenuGroupView {}
