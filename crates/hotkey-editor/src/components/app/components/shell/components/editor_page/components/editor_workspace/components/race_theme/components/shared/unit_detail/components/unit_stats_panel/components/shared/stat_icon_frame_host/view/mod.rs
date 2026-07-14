use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct StatIconFrameHostView {
    pub src: Asset,
    pub alt: String,
}

impl ddd::View for StatIconFrameHostView {}
