use super::view::PopoverActiveCategoryTabView;
use dioxus::prelude::*;

/// The selected popover tab's inputs: its caption and the select handler.
#[derive(Props, Clone, PartialEq)]
pub struct PopoverActiveCategoryTabModel {
    pub label: String,
    pub on_click: EventHandler<MouseEvent>,
}

impl From<&PopoverActiveCategoryTabView> for PopoverActiveCategoryTabModel {
    fn from(view: &PopoverActiveCategoryTabView) -> Self {
        let PopoverActiveCategoryTabView { label, on_click } = view.clone();
        Self { label, on_click }
    }
}

impl ddd::Model for PopoverActiveCategoryTabModel {
    type View = PopoverActiveCategoryTabView;
}
