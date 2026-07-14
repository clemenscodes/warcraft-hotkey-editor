use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BurgerDrawerView {
    pub on_close: EventHandler<MouseEvent>,
}

impl ddd::View for BurgerDrawerView {}
