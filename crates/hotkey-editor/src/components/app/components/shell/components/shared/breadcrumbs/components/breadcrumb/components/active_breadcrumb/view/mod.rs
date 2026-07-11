use dioxus::prelude::*;

/// The published `View` contract mirroring [`ActiveBreadcrumbModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ActiveBreadcrumbView {
    pub label: String,
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for ActiveBreadcrumbView {}
