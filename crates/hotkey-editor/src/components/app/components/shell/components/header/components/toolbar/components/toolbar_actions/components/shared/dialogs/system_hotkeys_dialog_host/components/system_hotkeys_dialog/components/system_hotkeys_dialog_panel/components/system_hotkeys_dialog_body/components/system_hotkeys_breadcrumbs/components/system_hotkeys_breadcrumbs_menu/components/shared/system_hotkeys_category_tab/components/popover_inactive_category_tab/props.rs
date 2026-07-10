use super::view::PopoverInactiveCategoryTabView;
use dioxus::prelude::*;

/// An unselected popover tab's inputs: its caption and the select handler.
#[derive(Props, Clone, PartialEq)]
pub struct PopoverInactiveCategoryTabProps {
    pub label: String,
    pub on_click: EventHandler<MouseEvent>,
}

impl From<&PopoverInactiveCategoryTabView> for PopoverInactiveCategoryTabProps {
    fn from(view: &PopoverInactiveCategoryTabView) -> Self {
        let PopoverInactiveCategoryTabView { label, on_click } = view.clone();
        Self { label, on_click }
    }
}

impl ddd::Props for PopoverInactiveCategoryTabProps {
    type View = PopoverInactiveCategoryTabView;
}
