use super::view::IdleMobileCategoryTabView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct IdleMobileCategoryTabModel {
    pub label: &'static str,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&IdleMobileCategoryTabView> for IdleMobileCategoryTabModel {
    fn from(view: &IdleMobileCategoryTabView) -> Self {
        let IdleMobileCategoryTabView { label, onclick } = view.clone();
        Self { label, onclick }
    }
}

impl ddd::Model for IdleMobileCategoryTabModel {
    type View = IdleMobileCategoryTabView;
}
