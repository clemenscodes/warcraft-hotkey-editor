use dioxus::prelude::*;

/// The published `View` contract mirroring [`ActiveMobileCategoryTabModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ActiveMobileCategoryTabView {
    pub label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for ActiveMobileCategoryTabView {}
