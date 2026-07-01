use super::super::burger_menu_group::BurgerMenuGroupProps;
use super::super::burger_menu_item::BurgerMenuItemProps;
use dioxus::prelude::*;

/// The drawer's scrolling content: the primary Grid Layout row on top, then the
/// grouped file actions.
#[derive(Props, Clone, PartialEq)]
pub struct BurgerDrawerBodyProps {
    pub layout: BurgerMenuItemProps,
    pub items: Vec<BurgerMenuItemProps>,
}

impl From<&BurgerDrawerBodyProps> for BurgerMenuGroupProps {
    fn from(props: &BurgerDrawerBodyProps) -> Self {
        let items = props.items.clone();
        Self { items }
    }
}
