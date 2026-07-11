use super::view::PopoverInactiveCategoryTabView;
use dioxus::prelude::*;

/// An unselected popover tab's inputs: its caption and the select handler.
#[derive(Props, Clone, PartialEq)]
pub struct PopoverInactiveCategoryTabModel {
    pub label: String,
    pub on_click: EventHandler<MouseEvent>,
}

impl From<&PopoverInactiveCategoryTabView> for PopoverInactiveCategoryTabModel {
    fn from(view: &PopoverInactiveCategoryTabView) -> Self {
        let PopoverInactiveCategoryTabView { label, on_click } = view.clone();
        Self { label, on_click }
    }
}

impl ddd::Model for PopoverInactiveCategoryTabModel {
    type View = PopoverInactiveCategoryTabView;
}
