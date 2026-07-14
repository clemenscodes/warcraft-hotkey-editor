use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct IdleBreadcrumbView {
    pub label: String,
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for IdleBreadcrumbView {}
