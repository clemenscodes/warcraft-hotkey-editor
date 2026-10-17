use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BurgerDownloadItemView {
    pub on_close: EventHandler<()>,
}

impl ddd::Layered for BurgerDownloadItemView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BurgerDownloadItemView {}
