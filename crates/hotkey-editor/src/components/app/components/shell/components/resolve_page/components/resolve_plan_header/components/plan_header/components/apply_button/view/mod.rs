use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ApplyButtonView {
    pub running: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for ApplyButtonView {}
