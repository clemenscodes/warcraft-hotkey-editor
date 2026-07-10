use super::view::BurgerMenuItemLabelView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerMenuItemLabelProps {
    pub text: String,
}

impl From<&BurgerMenuItemLabelView> for BurgerMenuItemLabelProps {
    fn from(view: &BurgerMenuItemLabelView) -> Self {
        let BurgerMenuItemLabelView { text } = view.clone();
        Self { text }
    }
}

impl ddd::Props for BurgerMenuItemLabelProps {
    type View = BurgerMenuItemLabelView;
}
