use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ActiveMobileCategoryTabView {
    pub label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for ActiveMobileCategoryTabView {}
