use super::view::SearchDialogFiltersView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq, Default)]
pub struct SearchDialogFiltersModel {
    pub open: bool,
}

impl From<&SearchDialogFiltersView> for SearchDialogFiltersModel {
    fn from(view: &SearchDialogFiltersView) -> Self {
        let SearchDialogFiltersView { open } = view.clone();
        Self { open }
    }
}

impl ddd::Model for SearchDialogFiltersModel {
    type View = SearchDialogFiltersView;
}
