use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BurgerResolveItemView {
    pub on_close: EventHandler<MouseEvent>,
}

impl ddd::Layered for BurgerResolveItemView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BurgerResolveItemView {}
