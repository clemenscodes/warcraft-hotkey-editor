use super::view::BurgerCloseView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerCloseModel {
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&BurgerCloseView> for BurgerCloseModel {
    fn from(view: &BurgerCloseView) -> Self {
        let BurgerCloseView { onclick } = view.clone();
        Self { onclick }
    }
}

impl ddd::Model for BurgerCloseModel {
    type View = BurgerCloseView;
}
