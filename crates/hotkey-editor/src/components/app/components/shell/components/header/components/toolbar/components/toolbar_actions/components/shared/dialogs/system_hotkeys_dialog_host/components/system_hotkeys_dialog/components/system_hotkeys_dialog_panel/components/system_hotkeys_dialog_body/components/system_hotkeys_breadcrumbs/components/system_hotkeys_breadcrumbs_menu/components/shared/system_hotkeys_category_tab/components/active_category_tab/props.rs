use super::view::ActiveCategoryTabView;
use dioxus::prelude::*;

/// The selected tab's inputs: its caption and the select handler.
#[derive(Props, Clone, PartialEq)]
pub struct ActiveCategoryTabProps {
    pub label: String,
    pub on_click: EventHandler<MouseEvent>,
}

impl From<&ActiveCategoryTabView> for ActiveCategoryTabProps {
    fn from(view: &ActiveCategoryTabView) -> Self {
        let ActiveCategoryTabView { label, on_click } = view.clone();
        Self { label, on_click }
    }
}

impl ddd::Props for ActiveCategoryTabProps {
    type View = ActiveCategoryTabView;
}
