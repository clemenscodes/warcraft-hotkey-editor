use super::view::ActiveMobileCategoryTabView;
use dioxus::prelude::*;

/// The active mobile category tab's props: its label and the tap handler.
#[derive(Props, Clone, PartialEq)]
pub struct ActiveMobileCategoryTabModel {
    pub label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&ActiveMobileCategoryTabView> for ActiveMobileCategoryTabModel {
    fn from(view: &ActiveMobileCategoryTabView) -> Self {
        let ActiveMobileCategoryTabView { label, onclick } = view.clone();
        Self { label, onclick }
    }
}

impl ddd::Model for ActiveMobileCategoryTabModel {
    type View = ActiveMobileCategoryTabView;
}
