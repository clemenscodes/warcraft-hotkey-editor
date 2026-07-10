use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerDrawerHeaderProps {
    pub onclick: EventHandler<MouseEvent>,
}
