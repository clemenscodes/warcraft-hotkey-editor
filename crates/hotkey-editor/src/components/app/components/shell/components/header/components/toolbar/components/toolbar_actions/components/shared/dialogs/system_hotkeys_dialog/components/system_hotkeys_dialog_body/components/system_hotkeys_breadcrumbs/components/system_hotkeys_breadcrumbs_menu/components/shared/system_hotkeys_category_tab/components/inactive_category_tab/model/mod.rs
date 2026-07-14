use super::view::InactiveCategoryTabView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InactiveCategoryTabModel {
    pub label: String,
    pub on_click: EventHandler<MouseEvent>,
}

impl From<&InactiveCategoryTabView> for InactiveCategoryTabModel {
    fn from(view: &InactiveCategoryTabView) -> Self {
        let InactiveCategoryTabView { label, on_click } = view.clone();
        Self { label, on_click }
    }
}

impl ddd::Model for InactiveCategoryTabModel {
    type View = InactiveCategoryTabView;
}
