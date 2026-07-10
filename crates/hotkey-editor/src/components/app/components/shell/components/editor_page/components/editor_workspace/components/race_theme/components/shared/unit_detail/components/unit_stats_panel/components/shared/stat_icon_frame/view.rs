use dioxus::prelude::*;

/// The published `View` contract mirroring [`StatIconFrameProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct StatIconFrameView {
    pub src: Asset,
    pub alt: String,
}

impl ddd::View for StatIconFrameView {}
