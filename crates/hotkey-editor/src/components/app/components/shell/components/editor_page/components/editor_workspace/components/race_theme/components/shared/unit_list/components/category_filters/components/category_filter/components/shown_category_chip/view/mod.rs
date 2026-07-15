use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ShownCategoryChipView {
    pub label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for ShownCategoryChipView {}
