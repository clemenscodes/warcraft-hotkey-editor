use super::components::burger_close::BurgerCloseProps;
use dioxus::prelude::*;

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
