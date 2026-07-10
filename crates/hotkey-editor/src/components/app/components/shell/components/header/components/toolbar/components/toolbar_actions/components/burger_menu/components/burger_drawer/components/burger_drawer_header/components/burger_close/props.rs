use super::view::BurgerCloseView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerCloseProps {
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&BurgerCloseView> for BurgerCloseProps {
    fn from(view: &BurgerCloseView) -> Self {
        let BurgerCloseView { onclick } = view.clone();
        Self { onclick }
    }
}

impl ddd::Props for BurgerCloseProps {
    type View = BurgerCloseView;
}
