use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::logic::BurgerMenuRow;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerMenuGroupProps {
    pub items: Vec<BurgerMenuRow>,
}
