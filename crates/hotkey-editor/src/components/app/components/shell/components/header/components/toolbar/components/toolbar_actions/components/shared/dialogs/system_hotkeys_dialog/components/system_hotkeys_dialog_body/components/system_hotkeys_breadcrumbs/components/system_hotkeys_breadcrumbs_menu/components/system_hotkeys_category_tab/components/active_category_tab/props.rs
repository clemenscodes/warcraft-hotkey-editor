use dioxus::prelude::*;

/// The selected tab's inputs: its caption and the select handler.
#[derive(Props, Clone, PartialEq)]
pub struct ActiveCategoryTabProps {
    pub label: String,
    pub on_click: EventHandler<MouseEvent>,
}
