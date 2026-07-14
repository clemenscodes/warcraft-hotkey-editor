use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct InactiveCategoryTabView {
    pub label: String,
    pub on_click: EventHandler<MouseEvent>,
}

impl ddd::View for InactiveCategoryTabView {}
