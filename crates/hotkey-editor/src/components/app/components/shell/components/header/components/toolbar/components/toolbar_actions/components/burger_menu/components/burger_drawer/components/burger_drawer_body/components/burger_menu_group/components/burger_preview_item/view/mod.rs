use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BurgerPreviewItemView {
    pub on_close: EventHandler<()>,
}

impl ddd::Layered for BurgerPreviewItemView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for BurgerPreviewItemView {}
