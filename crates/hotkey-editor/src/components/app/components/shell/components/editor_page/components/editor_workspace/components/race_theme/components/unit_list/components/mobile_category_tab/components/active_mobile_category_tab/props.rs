use dioxus::prelude::*;

/// The active mobile category tab's props: its label, its kind's data attribute, and the
/// tap handler.
#[derive(Props, Clone, PartialEq)]
pub struct ActiveMobileCategoryTabProps {
    pub label: &'static str,
    pub kind_attr: &'static str,
    pub onclick: EventHandler<MouseEvent>,
}
