use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BurgerDrawerBodyView {
    pub on_close: EventHandler<MouseEvent>,
}

impl ddd::View for BurgerDrawerBodyView {}
