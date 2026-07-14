use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BurgerDrawerHeaderView {
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for BurgerDrawerHeaderView {}
