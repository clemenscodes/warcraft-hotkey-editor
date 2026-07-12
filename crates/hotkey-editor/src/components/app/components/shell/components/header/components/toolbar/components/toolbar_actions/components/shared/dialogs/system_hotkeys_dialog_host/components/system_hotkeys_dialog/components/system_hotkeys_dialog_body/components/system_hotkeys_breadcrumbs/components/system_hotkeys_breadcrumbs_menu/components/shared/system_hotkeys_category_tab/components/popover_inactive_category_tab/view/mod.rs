use dioxus::prelude::*;

/// The published `View` contract mirroring [`PopoverInactiveCategoryTabModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct PopoverInactiveCategoryTabView {
    pub label: String,
    pub on_click: EventHandler<MouseEvent>,
}

impl ddd::View for PopoverInactiveCategoryTabView {}
