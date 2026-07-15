use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SearchScopeBackdropView {
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for SearchScopeBackdropView {}
