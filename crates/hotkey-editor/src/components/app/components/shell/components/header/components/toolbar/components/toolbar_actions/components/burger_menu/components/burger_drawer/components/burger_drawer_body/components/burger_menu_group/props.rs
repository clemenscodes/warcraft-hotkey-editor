use super::super::shared::burger_menu_item::BurgerMenuItemProps;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerMenuGroupProps {
    pub items: Vec<BurgerMenuItemProps>,
}
