use dioxus::prelude::*;

/// The selected popover tab's inputs: its caption and the select handler.
#[derive(Props, Clone, PartialEq)]
pub struct PopoverActiveCategoryTabProps {
    pub label: String,
    pub on_click: EventHandler<MouseEvent>,
}
