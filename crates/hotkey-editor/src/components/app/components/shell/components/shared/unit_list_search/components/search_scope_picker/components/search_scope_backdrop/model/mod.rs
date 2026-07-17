use super::view::SearchScopeBackdropView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SearchScopeBackdropModel {
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&SearchScopeBackdropView> for SearchScopeBackdropModel {
    fn from(view: &SearchScopeBackdropView) -> Self {
        let SearchScopeBackdropView { onclick } = view.clone();
        Self { onclick }
    }
}

impl ddd::Model for SearchScopeBackdropModel {
    type View = SearchScopeBackdropView;
}
