use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BurgerSystemHotkeysItemView {
    pub on_close: EventHandler<()>,
}

impl ddd::Layered for BurgerSystemHotkeysItemView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BurgerSystemHotkeysItemView {}
