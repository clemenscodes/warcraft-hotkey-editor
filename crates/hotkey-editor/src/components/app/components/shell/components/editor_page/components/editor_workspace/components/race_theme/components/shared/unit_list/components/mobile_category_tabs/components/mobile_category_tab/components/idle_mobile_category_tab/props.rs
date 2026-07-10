use dioxus::prelude::*;

/// The idle mobile category tab's props: its label and the tap handler.
#[derive(Props, Clone, PartialEq)]
pub struct IdleMobileCategoryTabProps {
    pub label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
}
