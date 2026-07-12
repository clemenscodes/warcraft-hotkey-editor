use dioxus::prelude::*;

/// The published `View` contract mirroring [`DialogHeaderModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct DialogHeaderView {
    pub title: String,
    pub on_close: EventHandler<()>,
}

impl ddd::View for DialogHeaderView {}
