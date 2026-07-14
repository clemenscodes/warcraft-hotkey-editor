use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BurgerUndoItemView {
    pub on_close: EventHandler<MouseEvent>,
}

impl ddd::View for BurgerUndoItemView {}
