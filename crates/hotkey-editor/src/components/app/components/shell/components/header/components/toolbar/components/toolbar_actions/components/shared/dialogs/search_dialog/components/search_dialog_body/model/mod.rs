use super::view::SearchDialogBodyView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq, Default)]
pub struct SearchDialogBodyModel {}

impl From<&SearchDialogBodyView> for SearchDialogBodyModel {
    fn from(view: &SearchDialogBodyView) -> Self {
        let SearchDialogBodyView {} = view.clone();
        Self {}
    }
}

impl ddd::Model for SearchDialogBodyModel {
    type View = SearchDialogBodyView;
}
