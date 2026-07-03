use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerBackdropProps {
    pub onclick: EventHandler<MouseEvent>,
}
