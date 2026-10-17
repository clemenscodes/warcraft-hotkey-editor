use super::view::ActiveMobileCategoryTabView;
use dioxus::prelude::*;

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

impl ddd::Layered for ActiveMobileCategoryTabModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for ActiveMobileCategoryTabModel {
    type View = ActiveMobileCategoryTabView;
}
