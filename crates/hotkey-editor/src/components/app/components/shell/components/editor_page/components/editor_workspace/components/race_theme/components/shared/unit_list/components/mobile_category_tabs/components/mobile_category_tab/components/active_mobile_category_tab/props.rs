use dioxus::prelude::*;

/// The active mobile category tab's props: its label and the tap handler.
#[derive(Props, Clone, PartialEq)]
pub struct ActiveMobileCategoryTabProps {
    pub label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
}
