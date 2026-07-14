use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SecondaryButtonView {
    pub onclick: EventHandler<MouseEvent>,
    pub label: String,
}

impl ddd::View for SecondaryButtonView {}
