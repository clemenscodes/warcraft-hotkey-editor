use super::view::SearchDialogScrimView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SearchDialogScrimModel {
    pub onclick: EventHandler<MouseEvent>,
    pub leaving: bool,
}

impl From<&SearchDialogScrimView> for SearchDialogScrimModel {
    fn from(view: &SearchDialogScrimView) -> Self {
        let SearchDialogScrimView { onclick, leaving } = view.clone();
        Self { onclick, leaving }
    }
}

impl ddd::Layered for SearchDialogScrimModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for SearchDialogScrimModel {
    type View = SearchDialogScrimView;
}
