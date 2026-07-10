use super::view::BurgerDrawerHeaderView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerDrawerHeaderProps {
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&BurgerDrawerHeaderView> for BurgerDrawerHeaderProps {
    fn from(view: &BurgerDrawerHeaderView) -> Self {
        let BurgerDrawerHeaderView { onclick } = view.clone();
        Self { onclick }
    }
}

impl ddd::Props for BurgerDrawerHeaderProps {
    type View = BurgerDrawerHeaderView;
}
