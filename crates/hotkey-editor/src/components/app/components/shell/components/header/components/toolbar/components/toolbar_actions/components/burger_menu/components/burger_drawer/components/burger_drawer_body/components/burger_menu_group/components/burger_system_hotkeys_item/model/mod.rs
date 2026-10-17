use super::view::BurgerSystemHotkeysItemView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerSystemHotkeysItemModel {
    pub on_close: EventHandler<()>,
}

impl From<&BurgerSystemHotkeysItemView> for BurgerSystemHotkeysItemModel {
    fn from(view: &BurgerSystemHotkeysItemView) -> Self {
        let BurgerSystemHotkeysItemView { on_close } = view.clone();
        Self { on_close }
    }
}

impl ddd::Layered for BurgerSystemHotkeysItemModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for BurgerSystemHotkeysItemModel {
    type View = BurgerSystemHotkeysItemView;
}
