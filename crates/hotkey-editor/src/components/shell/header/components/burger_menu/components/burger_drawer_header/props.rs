use dioxus::prelude::*;

use super::super::burger_close::BurgerCloseProps;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerDrawerHeaderProps {
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&BurgerDrawerHeaderProps> for BurgerCloseProps {
    fn from(props: &BurgerDrawerHeaderProps) -> Self {
        let onclick = props.onclick;
        Self { onclick }
    }
}
