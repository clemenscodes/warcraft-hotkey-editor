use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BurgerRedoItemView {
    pub on_close: EventHandler<MouseEvent>,
}

impl ddd::View for BurgerRedoItemView {}
