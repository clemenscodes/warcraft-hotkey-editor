use super::view::BurgerMenuGroupView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerMenuGroupModel {
    pub on_close: EventHandler<MouseEvent>,
}

impl From<&BurgerMenuGroupView> for BurgerMenuGroupModel {
    fn from(view: &BurgerMenuGroupView) -> Self {
        let BurgerMenuGroupView { on_close } = view.clone();
        Self { on_close }
    }
}

impl ddd::Model for BurgerMenuGroupModel {
    type View = BurgerMenuGroupView;
}
