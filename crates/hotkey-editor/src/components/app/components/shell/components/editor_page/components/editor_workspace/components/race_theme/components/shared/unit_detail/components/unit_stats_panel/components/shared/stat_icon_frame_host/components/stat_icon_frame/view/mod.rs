use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct StatIconFrameView {
    pub src: Asset,
    pub alt: String,
}

impl ddd::View for StatIconFrameView {}
