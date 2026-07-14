use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BurgerCloseView {
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for BurgerCloseView {}
