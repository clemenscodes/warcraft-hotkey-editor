use dioxus::prelude::*;

/// An unselected tab's inputs: its caption and the select handler.
#[derive(Props, Clone, PartialEq)]
pub struct InactiveCategoryTabProps {
    pub label: String,
    pub on_click: EventHandler<MouseEvent>,
}
