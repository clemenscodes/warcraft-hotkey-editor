use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BurgerUploadItemView {
    pub on_close: EventHandler<()>,
}

impl ddd::Layered for BurgerUploadItemView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BurgerUploadItemView {}
