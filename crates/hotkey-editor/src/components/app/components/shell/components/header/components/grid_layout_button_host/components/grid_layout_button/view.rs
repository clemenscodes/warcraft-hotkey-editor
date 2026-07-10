use dioxus::prelude::*;

/// The published `View` contract mirroring [`GridLayoutButtonProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct GridLayoutButtonView {
    pub is_open: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for GridLayoutButtonView {}
