use dioxus::prelude::*;

/// An unselected popover tab's inputs: its caption and the select handler.
#[derive(Props, Clone, PartialEq)]
pub struct PopoverInactiveCategoryTabProps {
    pub label: String,
    pub on_click: EventHandler<MouseEvent>,
}
