use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BurgerLayoutItemView {
    pub on_close: EventHandler<()>,
}

impl ddd::Layered for BurgerLayoutItemView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BurgerLayoutItemView {}
