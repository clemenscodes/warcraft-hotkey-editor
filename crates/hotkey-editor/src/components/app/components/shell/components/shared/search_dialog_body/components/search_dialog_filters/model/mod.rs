use super::view::SearchDialogFiltersView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SearchDialogFiltersModel {
    pub leaving: bool,
}

impl From<&SearchDialogFiltersView> for SearchDialogFiltersModel {
    fn from(view: &SearchDialogFiltersView) -> Self {
        let SearchDialogFiltersView { leaving } = *view;
        Self { leaving }
    }
}

impl ddd::Layered for SearchDialogFiltersModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for SearchDialogFiltersModel {
    type View = SearchDialogFiltersView;
}
