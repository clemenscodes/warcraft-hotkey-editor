use dioxus::prelude::*;

/// The published `View` contract mirroring [`GridLayoutEditorButtonModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct GridLayoutEditorButtonView {
    pub is_open: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for GridLayoutEditorButtonView {}
