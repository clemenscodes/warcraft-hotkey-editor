use dioxus::prelude::*;

/// The published `View` contract mirroring [`StatIconImgProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct StatIconImgView {
    pub src: Asset,
    pub alt: String,
}

impl ddd::View for StatIconImgView {}
