use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SearchScopeOptionView {
    pub label: String,
    pub is_active: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for SearchScopeOptionView {}
