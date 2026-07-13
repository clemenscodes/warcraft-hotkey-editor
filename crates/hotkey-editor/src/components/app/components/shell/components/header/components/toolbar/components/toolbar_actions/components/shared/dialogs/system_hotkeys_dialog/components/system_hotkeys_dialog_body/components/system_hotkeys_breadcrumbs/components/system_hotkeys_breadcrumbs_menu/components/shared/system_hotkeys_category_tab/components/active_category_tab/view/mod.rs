use dioxus::prelude::*;

/// The published `View` contract mirroring [`ActiveCategoryTabModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ActiveCategoryTabView {
    pub label: String,
    pub on_click: EventHandler<MouseEvent>,
}

impl ddd::View for ActiveCategoryTabView {}
