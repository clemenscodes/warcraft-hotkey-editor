use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct PrimaryButtonView {
    pub onclick: EventHandler<MouseEvent>,
    pub label: String,
}

impl ddd::View for PrimaryButtonView {}
