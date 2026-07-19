use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct SearchDialogScrimView {
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for SearchDialogScrimView {}
