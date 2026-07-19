use super::view::SearchDialogScrimView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SearchDialogScrimModel {
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&SearchDialogScrimView> for SearchDialogScrimModel {
    fn from(view: &SearchDialogScrimView) -> Self {
        let SearchDialogScrimView { onclick } = view.clone();
        Self { onclick }
    }
}

impl ddd::Model for SearchDialogScrimModel {
    type View = SearchDialogScrimView;
}
