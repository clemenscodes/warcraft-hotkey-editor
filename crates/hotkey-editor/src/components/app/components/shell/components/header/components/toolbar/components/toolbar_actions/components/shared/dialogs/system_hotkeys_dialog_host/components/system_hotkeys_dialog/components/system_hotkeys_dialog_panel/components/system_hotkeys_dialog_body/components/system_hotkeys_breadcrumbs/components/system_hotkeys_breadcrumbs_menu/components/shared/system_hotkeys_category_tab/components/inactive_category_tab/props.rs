use super::view::InactiveCategoryTabView;
use dioxus::prelude::*;

/// An unselected tab's inputs: its caption and the select handler.
#[derive(Props, Clone, PartialEq)]
pub struct InactiveCategoryTabProps {
    pub label: String,
    pub on_click: EventHandler<MouseEvent>,
}

impl From<&InactiveCategoryTabView> for InactiveCategoryTabProps {
    fn from(view: &InactiveCategoryTabView) -> Self {
        let InactiveCategoryTabView { label, on_click } = view.clone();
        Self { label, on_click }
    }
}

impl ddd::Props for InactiveCategoryTabProps {
    type View = InactiveCategoryTabView;
}
