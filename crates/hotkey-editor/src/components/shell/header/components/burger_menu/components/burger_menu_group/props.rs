use dioxus::prelude::*;

use super::super::burger_menu_item::BurgerMenuItemProps;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerMenuGroupProps {
    pub items: Vec<BurgerMenuItemProps>,
}
