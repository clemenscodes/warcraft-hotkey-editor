use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct StatIconFrameHostView {
    pub src: Asset,
    pub alt: String,
}

impl ddd::Layered for StatIconFrameHostView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for StatIconFrameHostView {}
