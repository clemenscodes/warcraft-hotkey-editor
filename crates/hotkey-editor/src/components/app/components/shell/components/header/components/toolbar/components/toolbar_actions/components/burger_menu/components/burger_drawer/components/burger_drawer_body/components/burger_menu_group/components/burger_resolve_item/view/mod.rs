use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BurgerResolveItemView {
    pub on_close: EventHandler<MouseEvent>,
}

impl ddd::View for BurgerResolveItemView {}
