use dioxus::prelude::*;

/// The published `View` contract mirroring [`PopoverActiveCategoryTabProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct PopoverActiveCategoryTabView {
    pub label: String,
    pub on_click: EventHandler<MouseEvent>,
}

impl ddd::View for PopoverActiveCategoryTabView {}
