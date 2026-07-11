use super::view::BurgerDrawerHeaderView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerDrawerHeaderModel {
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&BurgerDrawerHeaderView> for BurgerDrawerHeaderModel {
    fn from(view: &BurgerDrawerHeaderView) -> Self {
        let BurgerDrawerHeaderView { onclick } = view.clone();
        Self { onclick }
    }
}

impl ddd::Model for BurgerDrawerHeaderModel {
    type View = BurgerDrawerHeaderView;
}
