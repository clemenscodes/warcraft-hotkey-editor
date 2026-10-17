use super::view::BurgerPreviewItemView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerPreviewItemModel {
    pub on_close: EventHandler<()>,
}

impl From<&BurgerPreviewItemView> for BurgerPreviewItemModel {
    fn from(view: &BurgerPreviewItemView) -> Self {
        let BurgerPreviewItemView { on_close } = view.clone();
        Self { on_close }
    }
}

impl ddd::Layered for BurgerPreviewItemModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for BurgerPreviewItemModel {
    type View = BurgerPreviewItemView;
}
