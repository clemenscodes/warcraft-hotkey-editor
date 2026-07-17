use super::view::SearchScopeOptionView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SearchScopeOptionModel {
    #[props(into)]
    pub label: String,
    pub is_active: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&SearchScopeOptionView> for SearchScopeOptionModel {
    fn from(view: &SearchScopeOptionView) -> Self {
        let SearchScopeOptionView {
            label,
            is_active,
            onclick,
        } = view.clone();
        Self {
            label,
            is_active,
            onclick,
        }
    }
}

impl ddd::Model for SearchScopeOptionModel {
    type View = SearchScopeOptionView;
}
