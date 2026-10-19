use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SearchScopeOptionView {
    pub label: String,
    pub is_active: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for SearchScopeOptionView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SearchScopeOptionView {}
