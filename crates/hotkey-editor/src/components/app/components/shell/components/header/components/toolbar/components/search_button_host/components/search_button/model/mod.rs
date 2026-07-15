use super::view::SearchButtonView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SearchButtonModel {
    pub aria_expanded: Option<bool>,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&SearchButtonView> for SearchButtonModel {
    fn from(view: &SearchButtonView) -> Self {
        let SearchButtonView {
            aria_expanded,
            onclick,
        } = view.clone();
        Self {
            aria_expanded,
            onclick,
        }
    }
}

impl ddd::Model for SearchButtonModel {
    type View = SearchButtonView;
}
