use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct InactiveResolveSectionTabView {
    pub label: String,
    pub count: usize,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for InactiveResolveSectionTabView {}
