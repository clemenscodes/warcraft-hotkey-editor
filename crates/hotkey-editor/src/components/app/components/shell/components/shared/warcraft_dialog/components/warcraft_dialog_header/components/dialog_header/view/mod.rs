use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct DialogHeaderView {
    pub title: String,
    pub on_close: EventHandler<()>,
}

impl ddd::View for DialogHeaderView {}
