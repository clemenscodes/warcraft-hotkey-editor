use super::view::BurgerMenuItemIconView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerMenuItemIconModel {
    pub svg: &'static str,
}

impl From<&BurgerMenuItemIconView> for BurgerMenuItemIconModel {
    fn from(view: &BurgerMenuItemIconView) -> Self {
        let BurgerMenuItemIconView { svg } = view.clone();
        Self { svg }
    }
}

impl ddd::Model for BurgerMenuItemIconModel {
    type View = BurgerMenuItemIconView;
}
