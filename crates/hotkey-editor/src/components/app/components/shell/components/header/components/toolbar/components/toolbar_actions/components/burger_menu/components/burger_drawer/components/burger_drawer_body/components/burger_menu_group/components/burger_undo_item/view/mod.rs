use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BurgerUndoItemView {
    pub on_close: EventHandler<MouseEvent>,
}

impl ddd::Layered for BurgerUndoItemView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BurgerUndoItemView {}
