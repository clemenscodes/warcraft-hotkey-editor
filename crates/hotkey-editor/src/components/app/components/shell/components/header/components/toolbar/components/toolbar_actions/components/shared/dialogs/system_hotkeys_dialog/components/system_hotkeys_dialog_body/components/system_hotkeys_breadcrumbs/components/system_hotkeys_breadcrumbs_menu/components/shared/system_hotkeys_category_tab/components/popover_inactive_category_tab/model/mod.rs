use super::view::PopoverInactiveCategoryTabView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct PopoverInactiveCategoryTabModel {
    pub label: String,
    pub on_click: EventHandler<MouseEvent>,
}

impl From<&PopoverInactiveCategoryTabView> for PopoverInactiveCategoryTabModel {
    fn from(view: &PopoverInactiveCategoryTabView) -> Self {
        let PopoverInactiveCategoryTabView { label, on_click } = view.clone();
        Self { label, on_click }
    }
}

impl ddd::Model for PopoverInactiveCategoryTabModel {
    type View = PopoverInactiveCategoryTabView;
}
