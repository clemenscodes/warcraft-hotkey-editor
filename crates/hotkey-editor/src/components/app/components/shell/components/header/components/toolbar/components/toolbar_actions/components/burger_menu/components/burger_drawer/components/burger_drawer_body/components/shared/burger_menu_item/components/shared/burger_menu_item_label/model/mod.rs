use super::view::BurgerMenuItemLabelView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerMenuItemLabelModel {
    pub text: String,
}

impl From<&BurgerMenuItemLabelView> for BurgerMenuItemLabelModel {
    fn from(view: &BurgerMenuItemLabelView) -> Self {
        let BurgerMenuItemLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Layered for BurgerMenuItemLabelModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for BurgerMenuItemLabelModel {
    type View = BurgerMenuItemLabelView;
}
