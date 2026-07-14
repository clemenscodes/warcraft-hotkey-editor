use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ApplyButtonView {
    pub on_apply: EventHandler<MouseEvent>,
}

impl ddd::View for ApplyButtonView {}
