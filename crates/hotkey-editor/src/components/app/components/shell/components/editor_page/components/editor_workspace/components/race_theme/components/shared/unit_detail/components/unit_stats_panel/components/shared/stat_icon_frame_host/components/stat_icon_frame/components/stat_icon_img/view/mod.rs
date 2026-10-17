use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct StatIconImgView {
    pub src: Asset,
    pub alt: String,
}

impl ddd::Layered for StatIconImgView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for StatIconImgView {}
