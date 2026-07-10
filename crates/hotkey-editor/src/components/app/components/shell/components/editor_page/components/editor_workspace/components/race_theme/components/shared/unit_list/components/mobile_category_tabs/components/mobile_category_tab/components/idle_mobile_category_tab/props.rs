use super::view::IdleMobileCategoryTabView;
use dioxus::prelude::*;

/// The idle mobile category tab's props: its label and the tap handler.
#[derive(Props, Clone, PartialEq)]
pub struct IdleMobileCategoryTabProps {
    pub label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&IdleMobileCategoryTabView> for IdleMobileCategoryTabProps {
    fn from(view: &IdleMobileCategoryTabView) -> Self {
        let IdleMobileCategoryTabView { label, onclick } = view.clone();
        Self { label, onclick }
    }
}

impl ddd::Props for IdleMobileCategoryTabProps {
    type View = IdleMobileCategoryTabView;
}
