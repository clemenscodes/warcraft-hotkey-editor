use super::view::BurgerMenuGroupView;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::burger_menu::presentation::BurgerMenuRow;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerMenuGroupModel {
    pub items: Vec<BurgerMenuRow>,
}

impl From<&BurgerMenuGroupView> for BurgerMenuGroupModel {
    fn from(view: &BurgerMenuGroupView) -> Self {
        let BurgerMenuGroupView { items } = view.clone();
        Self { items }
    }
}

impl ddd::Model for BurgerMenuGroupModel {
    type View = BurgerMenuGroupView;
}
