use super::view::SearchScopeTriggerView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SearchScopeTriggerModel {
    #[props(into)]
    pub label: String,
    pub is_open: bool,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&SearchScopeTriggerView> for SearchScopeTriggerModel {
    fn from(view: &SearchScopeTriggerView) -> Self {
        let SearchScopeTriggerView {
            label,
            is_open,
            onclick,
        } = view.clone();
        Self {
            label,
            is_open,
            onclick,
        }
    }
}

impl ddd::Model for SearchScopeTriggerModel {
    type View = SearchScopeTriggerView;
}
