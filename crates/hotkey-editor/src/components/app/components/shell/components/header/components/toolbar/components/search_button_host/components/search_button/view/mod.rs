use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SearchButtonView {
    pub aria_expanded: Option<bool>,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for SearchButtonView {}
