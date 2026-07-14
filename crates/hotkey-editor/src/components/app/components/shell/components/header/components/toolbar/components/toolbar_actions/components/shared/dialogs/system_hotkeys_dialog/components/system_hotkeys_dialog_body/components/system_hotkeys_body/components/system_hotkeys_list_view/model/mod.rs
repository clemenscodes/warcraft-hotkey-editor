use super::view::SystemHotkeysListViewView;
use dioxus::prelude::*;
use warcraft_api::SystemHotkeysCategory;

#[derive(Props, Clone, PartialEq)]
pub struct SystemHotkeysListViewModel {
    pub category: SystemHotkeysCategory,
}

impl From<&SystemHotkeysListViewView> for SystemHotkeysListViewModel {
    fn from(view: &SystemHotkeysListViewView) -> Self {
        let SystemHotkeysListViewView { category } = view.clone();
        Self { category }
    }
}

impl ddd::Model for SystemHotkeysListViewModel {
    type View = SystemHotkeysListViewView;
}
