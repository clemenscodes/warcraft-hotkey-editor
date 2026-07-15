use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct HiddenCategoryChipView {
    pub label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
}

impl ddd::View for HiddenCategoryChipView {}
