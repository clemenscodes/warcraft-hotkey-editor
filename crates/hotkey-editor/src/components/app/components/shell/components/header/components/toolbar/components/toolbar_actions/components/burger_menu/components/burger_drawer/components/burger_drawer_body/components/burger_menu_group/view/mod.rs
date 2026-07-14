use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BurgerMenuGroupView {
    pub on_close: EventHandler<MouseEvent>,
}

impl ddd::View for BurgerMenuGroupView {}
