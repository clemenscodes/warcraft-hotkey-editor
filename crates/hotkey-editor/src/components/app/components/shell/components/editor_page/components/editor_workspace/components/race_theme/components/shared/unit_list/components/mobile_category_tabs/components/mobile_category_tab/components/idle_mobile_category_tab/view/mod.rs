use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct IdleMobileCategoryTabView {
    pub label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for IdleMobileCategoryTabView {}
