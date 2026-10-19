use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SearchScopeBackdropView {
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::Layered for SearchScopeBackdropView {
    type Layer = ddd::PresentationLayer;
}

impl ddd::View for SearchScopeBackdropView {}
