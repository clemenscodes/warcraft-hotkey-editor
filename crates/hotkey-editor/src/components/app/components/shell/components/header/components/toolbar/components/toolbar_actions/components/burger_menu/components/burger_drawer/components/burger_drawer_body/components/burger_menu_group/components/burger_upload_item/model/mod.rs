use super::view::BurgerUploadItemView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerUploadItemModel {
    pub on_close: EventHandler<()>,
}

impl From<&BurgerUploadItemView> for BurgerUploadItemModel {
    fn from(view: &BurgerUploadItemView) -> Self {
        let BurgerUploadItemView { on_close } = view.clone();
        Self { on_close }
    }
}

impl ddd::Layered for BurgerUploadItemModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for BurgerUploadItemModel {
    type View = BurgerUploadItemView;
}
