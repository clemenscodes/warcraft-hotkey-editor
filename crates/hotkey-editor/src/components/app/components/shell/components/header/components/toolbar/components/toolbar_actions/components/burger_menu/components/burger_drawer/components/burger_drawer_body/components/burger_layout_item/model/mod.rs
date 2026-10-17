use super::view::BurgerLayoutItemView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerLayoutItemModel {
    pub on_close: EventHandler<()>,
}

impl From<&BurgerLayoutItemView> for BurgerLayoutItemModel {
    fn from(view: &BurgerLayoutItemView) -> Self {
        let BurgerLayoutItemView { on_close } = view.clone();
        Self { on_close }
    }
}

impl ddd::Layered for BurgerLayoutItemModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for BurgerLayoutItemModel {
    type View = BurgerLayoutItemView;
}
