use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BurgerBackdropView {
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for BurgerBackdropView {}
