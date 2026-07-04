use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerCloseProps {
    pub onclick: EventHandler<MouseEvent>,
}
