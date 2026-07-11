use dioxus::prelude::*;

/// The published `View` contract mirroring [`IdleBreadcrumbModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct IdleBreadcrumbView {
    pub label: String,
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for IdleBreadcrumbView {}
