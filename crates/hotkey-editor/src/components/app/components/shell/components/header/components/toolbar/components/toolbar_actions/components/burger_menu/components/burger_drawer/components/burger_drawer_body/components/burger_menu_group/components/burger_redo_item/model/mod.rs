use super::view::BurgerRedoItemView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerRedoItemModel {
    pub on_close: EventHandler<()>,
}

impl From<&BurgerRedoItemView> for BurgerRedoItemModel {
    fn from(view: &BurgerRedoItemView) -> Self {
        let BurgerRedoItemView { on_close } = view.clone();
        Self { on_close }
    }
}

impl ddd::Layered for BurgerRedoItemModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for BurgerRedoItemModel {
    type View = BurgerRedoItemView;
}
