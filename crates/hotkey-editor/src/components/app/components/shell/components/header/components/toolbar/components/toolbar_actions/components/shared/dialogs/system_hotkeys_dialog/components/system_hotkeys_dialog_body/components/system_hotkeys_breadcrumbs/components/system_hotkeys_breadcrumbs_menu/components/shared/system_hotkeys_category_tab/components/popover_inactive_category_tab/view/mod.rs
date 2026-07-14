use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct PopoverInactiveCategoryTabView {
    pub label: String,
    pub on_click: EventHandler<MouseEvent>,
}

impl ddd::View for PopoverInactiveCategoryTabView {}
