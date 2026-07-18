use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SearchConfigButtonView {
    pub label: String,
    pub open: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for SearchConfigButtonView {}
