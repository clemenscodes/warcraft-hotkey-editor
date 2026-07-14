use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ActiveBreadcrumbView {
    pub label: String,
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for ActiveBreadcrumbView {}
