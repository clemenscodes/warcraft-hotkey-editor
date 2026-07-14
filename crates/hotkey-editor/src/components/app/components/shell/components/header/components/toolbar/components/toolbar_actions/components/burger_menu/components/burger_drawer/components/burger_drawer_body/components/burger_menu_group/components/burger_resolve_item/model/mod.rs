use super::view::BurgerResolveItemView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerResolveItemModel {
    pub on_close: EventHandler<MouseEvent>,
}

impl From<&BurgerResolveItemView> for BurgerResolveItemModel {
    fn from(view: &BurgerResolveItemView) -> Self {
        let BurgerResolveItemView { on_close } = view.clone();
        Self { on_close }
    }
}

impl ddd::Model for BurgerResolveItemModel {
    type View = BurgerResolveItemView;
}
