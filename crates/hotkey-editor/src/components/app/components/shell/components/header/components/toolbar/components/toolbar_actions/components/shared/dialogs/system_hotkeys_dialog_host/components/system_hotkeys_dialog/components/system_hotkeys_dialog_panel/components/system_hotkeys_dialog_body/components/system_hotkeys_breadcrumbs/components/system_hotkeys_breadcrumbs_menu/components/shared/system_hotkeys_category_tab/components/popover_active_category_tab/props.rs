use super::view::PopoverActiveCategoryTabView;
use dioxus::prelude::*;

/// The selected popover tab's inputs: its caption and the select handler.
#[derive(Props, Clone, PartialEq)]
pub struct PopoverActiveCategoryTabProps {
    pub label: String,
    pub on_click: EventHandler<MouseEvent>,
}

impl From<&PopoverActiveCategoryTabView> for PopoverActiveCategoryTabProps {
    fn from(view: &PopoverActiveCategoryTabView) -> Self {
        let PopoverActiveCategoryTabView { label, on_click } = view.clone();
        Self { label, on_click }
    }
}

impl ddd::Props for PopoverActiveCategoryTabProps {
    type View = PopoverActiveCategoryTabView;
}
