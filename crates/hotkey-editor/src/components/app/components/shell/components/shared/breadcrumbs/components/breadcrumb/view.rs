use dioxus::prelude::*;

/// The published `View` contract mirroring [`BreadcrumbProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct BreadcrumbView {
    pub label: String,
    pub count: usize,
    pub active: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for BreadcrumbView {}
