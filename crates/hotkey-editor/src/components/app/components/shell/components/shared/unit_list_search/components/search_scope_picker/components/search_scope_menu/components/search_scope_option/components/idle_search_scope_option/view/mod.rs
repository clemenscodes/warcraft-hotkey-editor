use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct IdleSearchScopeOptionView {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for IdleSearchScopeOptionView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for IdleSearchScopeOptionView {}
