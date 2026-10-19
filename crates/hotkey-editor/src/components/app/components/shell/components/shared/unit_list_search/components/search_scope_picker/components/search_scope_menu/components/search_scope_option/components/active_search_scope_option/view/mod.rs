use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ActiveSearchScopeOptionView {
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for ActiveSearchScopeOptionView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for ActiveSearchScopeOptionView {}
