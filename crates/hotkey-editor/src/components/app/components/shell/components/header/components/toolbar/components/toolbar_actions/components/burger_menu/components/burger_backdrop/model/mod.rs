use super::view::BurgerBackdropView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerBackdropModel {
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&BurgerBackdropView> for BurgerBackdropModel {
    fn from(view: &BurgerBackdropView) -> Self {
        let BurgerBackdropView { onclick } = view.clone();
        Self { onclick }
    }
}

impl ddd::Model for BurgerBackdropModel {
    type View = BurgerBackdropView;
}
