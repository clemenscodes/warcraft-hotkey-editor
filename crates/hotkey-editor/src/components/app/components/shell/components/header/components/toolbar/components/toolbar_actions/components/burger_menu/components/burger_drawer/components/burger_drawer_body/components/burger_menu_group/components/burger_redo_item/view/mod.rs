use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BurgerRedoItemView {
    pub on_close: EventHandler<()>,
}

impl ddd::Layered for BurgerRedoItemView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BurgerRedoItemView {}
