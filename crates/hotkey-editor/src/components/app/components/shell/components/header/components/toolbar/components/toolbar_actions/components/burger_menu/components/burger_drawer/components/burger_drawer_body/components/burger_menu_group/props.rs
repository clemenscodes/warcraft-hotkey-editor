use super::view::BurgerMenuGroupView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::logic::BurgerMenuRow;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerMenuGroupProps {
    pub items: Vec<BurgerMenuRow>,
}

impl From<&BurgerMenuGroupView> for BurgerMenuGroupProps {
    fn from(view: &BurgerMenuGroupView) -> Self {
        let BurgerMenuGroupView { items } = view.clone();
        Self { items }
    }
}

impl ddd::Props for BurgerMenuGroupProps {
    type View = BurgerMenuGroupView;
}
