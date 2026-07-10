use super::view::BurgerMenuItemIconView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerMenuItemIconProps {
    pub svg: &'static str,
}

impl From<&BurgerMenuItemIconView> for BurgerMenuItemIconProps {
    fn from(view: &BurgerMenuItemIconView) -> Self {
        let BurgerMenuItemIconView { svg } = view.clone();
        Self { svg }
    }
}

impl ddd::Props for BurgerMenuItemIconProps {
    type View = BurgerMenuItemIconView;
}
