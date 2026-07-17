use super::view::SearchDialogFiltersView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq, Default)]
pub struct SearchDialogFiltersModel {}

impl From<&SearchDialogFiltersView> for SearchDialogFiltersModel {
    fn from(view: &SearchDialogFiltersView) -> Self {
        let SearchDialogFiltersView {} = view.clone();
        Self {}
    }
}

impl ddd::Model for SearchDialogFiltersModel {
    type View = SearchDialogFiltersView;
}
