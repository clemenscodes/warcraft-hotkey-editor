use super::view::BurgerDownloadItemView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BurgerDownloadItemModel {
    pub on_close: EventHandler<()>,
}

impl From<&BurgerDownloadItemView> for BurgerDownloadItemModel {
    fn from(view: &BurgerDownloadItemView) -> Self {
        let BurgerDownloadItemView { on_close } = view.clone();
        Self { on_close }
    }
}

impl ddd::Layered for BurgerDownloadItemModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for BurgerDownloadItemModel {
    type View = BurgerDownloadItemView;
}
