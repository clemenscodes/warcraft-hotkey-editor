use dioxus::prelude::*;

/// The published `View` contract mirroring [`DialogCloseProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct DialogCloseView {
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for DialogCloseView {}
