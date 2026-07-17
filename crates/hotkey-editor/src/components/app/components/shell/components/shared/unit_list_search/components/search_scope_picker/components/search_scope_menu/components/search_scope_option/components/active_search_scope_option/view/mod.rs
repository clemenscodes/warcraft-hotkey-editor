use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ActiveSearchScopeOptionView {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for ActiveSearchScopeOptionView {}
