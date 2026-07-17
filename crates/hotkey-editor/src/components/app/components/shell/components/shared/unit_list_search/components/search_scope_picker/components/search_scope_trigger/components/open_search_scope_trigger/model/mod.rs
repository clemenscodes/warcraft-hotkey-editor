use super::view::OpenSearchScopeTriggerView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct OpenSearchScopeTriggerModel {
    #[props(into)]
    pub label: String,
    pub onclick: EventHandler<MouseEvent>,
}

impl From<&OpenSearchScopeTriggerView> for OpenSearchScopeTriggerModel {
    fn from(view: &OpenSearchScopeTriggerView) -> Self {
        let OpenSearchScopeTriggerView { label, onclick } = view.clone();
        Self { label, onclick }
    }
}

impl ddd::Model for OpenSearchScopeTriggerModel {
    type View = OpenSearchScopeTriggerView;
}
