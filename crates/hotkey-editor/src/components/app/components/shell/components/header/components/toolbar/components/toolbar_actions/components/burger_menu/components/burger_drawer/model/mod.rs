use super::view::BurgerDrawerView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerDrawerModel {
    pub on_close: EventHandler<MouseEvent>,
}

impl From<&BurgerDrawerView> for BurgerDrawerModel {
    fn from(view: &BurgerDrawerView) -> Self {
        let BurgerDrawerView { on_close } = view.clone();
        Self { on_close }
    }
}

impl ddd::Model for BurgerDrawerModel {
    type View = BurgerDrawerView;
}
