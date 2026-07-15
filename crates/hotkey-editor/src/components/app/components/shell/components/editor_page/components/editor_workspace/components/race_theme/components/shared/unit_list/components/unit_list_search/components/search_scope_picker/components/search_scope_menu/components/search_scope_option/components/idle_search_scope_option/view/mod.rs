use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct IdleSearchScopeOptionView {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for IdleSearchScopeOptionView {}
