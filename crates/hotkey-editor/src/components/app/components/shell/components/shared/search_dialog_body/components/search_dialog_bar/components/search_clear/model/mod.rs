use super::view::SearchClearView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SearchClearModel {
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&SearchClearView> for SearchClearModel {
    fn from(view: &SearchClearView) -> Self {
        let SearchClearView { onclick } = view.clone();
        Self { onclick }
    }
}

impl ddd::Layered for SearchClearModel {
    type Layer = ddd::PresentationLayer;
}

impl ddd::Model for SearchClearModel {
    type View = SearchClearView;
}
