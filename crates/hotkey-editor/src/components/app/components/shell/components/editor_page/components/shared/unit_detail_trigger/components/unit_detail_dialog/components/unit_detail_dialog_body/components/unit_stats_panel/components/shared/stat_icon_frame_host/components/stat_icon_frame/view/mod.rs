use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct StatIconFrameView {
    pub src: Asset,
    pub alt: String,
}

impl ddd::Layered for StatIconFrameView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for StatIconFrameView {}
